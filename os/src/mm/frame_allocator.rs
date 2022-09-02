//! 物理页(frame)申请器

use crate::config::MEMORY_END;
use crate::mm::address::{PhysAddr, PhysPageNum};
use crate::sync::UPSafeCell;
use alloc::vec::Vec;
use core::fmt;
use core::fmt::{Debug, Formatter};
use lazy_static::lazy_static;

/// 使用tracker包装物理页
pub struct FrameTracker {
    pub ppn: PhysPageNum,
}

impl FrameTracker {
    /// 初始化 frame, 把内存清零
    pub fn new(ppn: PhysPageNum) -> Self {
        trace!("alloc a new frame tracker {:?}", ppn);
        // page cleaning
        let bytes_array = ppn.get_bytes_array();
        for i in bytes_array {
            *i = 0;
        }
        Self { ppn }
    }
}

/// Debugging
impl Debug for FrameTracker {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("FrameTracker:PPN={:#x}", self.ppn.0))
    }
}

/// 实现drop方法, tracker被回收时自动调用frame_dealloc,回收物理页
impl Drop for FrameTracker {
    fn drop(&mut self) {
        trace!("dealloc frame tracker {:?}", self.ppn);
        frame_dealloc(self.ppn);
    }
}

/// 物理页申请器的trait
trait FrameAllocator {
    fn new() -> Self;
    /// 申请一页物理地址
    fn alloc(&mut self) -> Option<PhysPageNum>;
    /// 释放一页物理地址
    fn dealloc(&mut self, ppn: PhysPageNum);
}

/// 栈式物理页帧管理策略
pub struct StackFrameAllocator {
    // 空闲内存的起始物理页号
    current: usize,
    // 空闲内存的结束物理页号
    end: usize,
    // 回收列表
    recycled: Vec<usize>,
}

impl StackFrameAllocator {
    pub fn init(&mut self, l: PhysPageNum, r: PhysPageNum) {
        info!("frame memory left {:?}, right {:?}", l, r);
        self.current = l.0;
        self.end = r.0;
        info!("last {} Physical Frames.", self.end - self.current);
    }
}

impl FrameAllocator for StackFrameAllocator {
    fn new() -> Self {
        Self {
            current: 0,
            end: 0,
            recycled: Vec::new(),
        }
    }

    fn alloc(&mut self) -> Option<PhysPageNum> {
        if let Some(ppn) = self.recycled.pop() {
            // 优先从回收列表里pop一个
            Some(ppn.into())
        } else {
            // 回收列表里没有检查还有没有内存
            if self.current == self.end {
                None
            } else {
                // 还有从头部取一个
                self.current += 1;
                Some((self.current - 1).into())
            }
        }
    }

    fn dealloc(&mut self, ppn: PhysPageNum) {
        let ppn = ppn.0;
        // validity check
        if ppn >= self.current || self.recycled.iter().find(|&v| *v == ppn).is_some() {
            // 不符合alloc的内存条件啊.
            panic!("Frame ppn={:#x} has not been allocated!", ppn);
        }
        // recycle
        self.recycled.push(ppn);
    }
}

/// frame 申请器实现选择 栈式
type FrameAllocatorImpl = StackFrameAllocator;

lazy_static! {
    /// 全局对象
    pub static ref FRAME_ALLOCATOR: UPSafeCell<FrameAllocatorImpl> = unsafe {
        UPSafeCell::new(FrameAllocatorImpl::new())
    };
}

/// 初始化 frame allocator
pub fn init_frame_allocator() {
    extern "C" {
        /// 获取物理地址start, 定义于linker.ld
        fn ekernel();
    }
    info!("init frame allocator");
    // 根据物理地址初始化 frame_allocator
    FRAME_ALLOCATOR.exclusive_access().init(
        PhysAddr::from(ekernel as usize).ceil(),
        PhysAddr::from(MEMORY_END).floor(),
    );
}

// ----------------------- mod public functions -----------------------

/// 申请一块frame
pub fn frame_alloc() -> Option<FrameTracker> {
    FRAME_ALLOCATOR
        .exclusive_access()
        .alloc()
        .map(|ppn| FrameTracker::new(ppn))
}

/// 释放一块frame
pub fn frame_dealloc(ppn: PhysPageNum) {
    FRAME_ALLOCATOR.exclusive_access().dealloc(ppn);
}

// ----------------------- unit tests -----------------------

#[allow(unused)]
pub fn frame_allocator_test() {
    let mut v: Vec<FrameTracker> = Vec::new();
    for i in 0..5 {
        let frame = frame_alloc().unwrap();
        debug!("{:?}", frame);
        v.push(frame);
    }
    v.clear();
    for i in 0..5 {
        let frame = frame_alloc().unwrap();
        debug!("{:?}", frame);
        v.push(frame);
    }
    drop(v);
    debug!("frame_allocator_test passed!");
}
