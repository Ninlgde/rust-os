//! 管道的实现
use super::File;
use crate::mm::UserBuffer;
use crate::sync::UPSafeCell;
use alloc::sync::{Arc, Weak};

use crate::task::suspend_current_and_run_next;

/// 管道结构体
pub struct Pipe {
    /// 可读?
    readable: bool,
    /// 可写?
    writable: bool,
    /// 双端环形队列.
    buffer: Arc<UPSafeCell<PipeRingBuffer>>,
}

impl Pipe {
    /// 创建读端pipe
    pub fn read_end_with_buffer(buffer: Arc<UPSafeCell<PipeRingBuffer>>) -> Self {
        Self {
            readable: true,
            writable: false,
            buffer,
        }
    }
    /// 创建写端pipe
    pub fn write_end_with_buffer(buffer: Arc<UPSafeCell<PipeRingBuffer>>) -> Self {
        Self {
            readable: false,
            writable: true,
            buffer,
        }
    }
}

/// 队列大小
const RING_BUFFER_SIZE: usize = 32;

#[derive(Copy, Clone, PartialEq)]
/// 队列状态
enum RingBufferStatus {
    /// 满
    Full,
    /// 空
    Empty,
    /// 其他
    Normal,
}

/// pipe 环形双端队列
pub struct PipeRingBuffer {
    /// 队列数组
    arr: [u8; RING_BUFFER_SIZE],
    /// 头
    head: usize,
    /// 尾
    tail: usize,
    /// 队列状态
    status: RingBufferStatus,
    /// 写端的弱引用
    write_end: Option<Weak<Pipe>>,
}

impl PipeRingBuffer {
    /// create a new pipe ring buffer
    pub fn new() -> Self {
        Self {
            arr: [0; RING_BUFFER_SIZE],
            head: 0,
            tail: 0,
            status: RingBufferStatus::Empty,
            write_end: None,
        }
    }
    /// 设置写端
    pub fn set_write_end(&mut self, write_end: &Arc<Pipe>) {
        self.write_end = Some(Arc::downgrade(write_end));
    }
    /// 写入数据
    pub fn write_byte(&mut self, byte: u8) {
        self.status = RingBufferStatus::Normal;
        self.arr[self.tail] = byte;
        self.tail = (self.tail + 1) % RING_BUFFER_SIZE;
        if self.tail == self.head {
            self.status = RingBufferStatus::Full;
        }
    }
    /// 读取数据
    pub fn read_byte(&mut self) -> u8 {
        self.status = RingBufferStatus::Normal;
        let c = self.arr[self.head];
        self.head = (self.head + 1) % RING_BUFFER_SIZE;
        if self.head == self.tail {
            self.status = RingBufferStatus::Empty;
        }
        c
    }
    /// 是否有可读内容
    pub fn available_read(&self) -> usize {
        if self.status == RingBufferStatus::Empty {
            0
        } else if self.tail > self.head {
            self.tail - self.head
        } else {
            self.tail + RING_BUFFER_SIZE - self.head
        }
    }
    /// 是否有写入空间
    pub fn available_write(&self) -> usize {
        if self.status == RingBufferStatus::Full {
            0
        } else {
            RING_BUFFER_SIZE - self.available_read()
        }
    }
    /// 判断管道的所有写端是否都被关闭了
    pub fn all_write_ends_closed(&self) -> bool {
        self.write_end.as_ref().unwrap().upgrade().is_none()
    }
}

/// Return (read_end, write_end)
pub fn make_pipe() -> (Arc<Pipe>, Arc<Pipe>) {
    let buffer = Arc::new(unsafe { UPSafeCell::new(PipeRingBuffer::new()) });
    let read_end = Arc::new(Pipe::read_end_with_buffer(buffer.clone()));
    let write_end = Arc::new(Pipe::write_end_with_buffer(buffer.clone()));
    buffer.exclusive_access().set_write_end(&write_end);
    (read_end, write_end)
}

impl File for Pipe {
    /// 可读
    fn readable(&self) -> bool {
        self.readable
    }
    /// 可写
    fn writable(&self) -> bool {
        self.writable
    }
    /// 从 `ring_buffer` 中读取,写入 `buf`中
    fn read(&self, buf: UserBuffer) -> usize {
        assert!(self.readable());
        let mut buf_iter = buf.into_iter();
        let mut read_size = 0usize;
        loop {
            let mut ring_buffer = self.buffer.exclusive_access();
            let loop_read = ring_buffer.available_read();
            if loop_read == 0 {
                if ring_buffer.all_write_ends_closed() {
                    return read_size;
                }
                drop(ring_buffer);
                suspend_current_and_run_next();
                continue;
            }
            // read at most loop_read bytes
            for _ in 0..loop_read {
                if let Some(byte_ref) = buf_iter.next() {
                    unsafe {
                        *byte_ref = ring_buffer.read_byte();
                    }
                    read_size += 1;
                } else {
                    return read_size;
                }
            }
        }
    }
    /// 从 `buf` 中读取, 写入 `ring_buffer`中
    fn write(&self, buf: UserBuffer) -> usize {
        assert!(self.writable());
        let mut buf_iter = buf.into_iter();
        let mut write_size = 0usize;
        loop {
            let mut ring_buffer = self.buffer.exclusive_access();
            let loop_write = ring_buffer.available_write();
            if loop_write == 0 {
                drop(ring_buffer);
                suspend_current_and_run_next();
                continue;
            }
            // write at most loop_write bytes
            for _ in 0..loop_write {
                if let Some(byte_ref) = buf_iter.next() {
                    ring_buffer.write_byte(unsafe { *byte_ref });
                    write_size += 1;
                } else {
                    return write_size;
                }
            }
        }
    }
}
