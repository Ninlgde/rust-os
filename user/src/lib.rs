#![no_std]
#![feature(linkage)]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]

extern crate alloc;

#[macro_use]
pub mod console;
#[macro_use]
pub mod logging;
mod lang_items;
mod syscall;

use alloc::vec::Vec;
use bitflags::bitflags;
use buddy_system_allocator::LockedHeap;

const USER_HEAP_SIZE: usize = 16384;

static mut HEAP_SPACE: [u8; USER_HEAP_SIZE] = [0; USER_HEAP_SIZE];

#[global_allocator]
static HEAP: LockedHeap = LockedHeap::empty();

#[alloc_error_handler]
pub fn handle_alloc_error(layout: core::alloc::Layout) -> ! {
    panic!("Heap allocation error, layout = {:?}", layout);
}

#[no_mangle]
#[link_section = ".text.entry"]
pub extern "C" fn _start(argc: usize, argv: usize) -> ! {
    logging::init();
    unsafe {
        HEAP.lock()
            .init(HEAP_SPACE.as_ptr() as usize, USER_HEAP_SIZE);
    }
    let mut v: Vec<&'static str> = Vec::new();
    for i in 0..argc {
        let str_start =
            unsafe { ((argv + i * core::mem::size_of::<usize>()) as *const usize).read_volatile() };
        let len = (0usize..)
            .find(|i| unsafe { ((str_start + *i) as *const u8).read_volatile() == 0 })
            .unwrap();
        v.push(
            core::str::from_utf8(unsafe {
                core::slice::from_raw_parts(str_start as *const u8, len)
            })
            .unwrap(),
        );
    }
    exit(main(argc, v.as_slice()));
    panic!("unreachable after sys_exit!");
}

#[linkage = "weak"]
#[no_mangle]
fn main(_argc: usize, _argv: &[&str]) -> i32 {
    panic!("Cannot find main!");
}

use syscall::*;

bitflags! {
    pub struct OpenFlags: u32 {
        const RDONLY = 0;
        const WRONLY = 1 << 0;
        const RDWR = 1 << 1;
        const CREATE = 1 << 9;
        const TRUNC = 1 << 10;
    }
}

pub fn dup(fd: usize) -> isize {
    sys_dup(fd)
}

pub fn open(path: &str, flags: OpenFlags) -> isize {
    sys_open(path, flags.bits)
}

pub fn close(fd: usize) -> isize {
    sys_close(fd)
}

pub fn pipe(pipe_fd: &mut [usize]) -> isize {
    sys_pipe(pipe_fd)
}

pub fn ls(path: &str) -> isize {
    sys_ls(path)
}

pub fn read(fd: usize, buf: &mut [u8]) -> isize {
    sys_read(fd, buf)
}

pub fn write(fd: usize, buf: &[u8]) -> isize {
    sys_write(fd, buf)
}

pub fn exit(exit_code: i32) -> isize {
    sys_exit(exit_code)
}

pub fn yield_() -> isize {
    sys_yield()
}

pub fn get_time() -> isize {
    sys_get_time()
}

pub fn getpid() -> isize {
    sys_getpid()
}

pub fn fork() -> isize {
    sys_fork()
}

pub fn exec(path: &str, args: &[*const u8]) -> isize {
    sys_exec(path, args)
}

/// 任意子进程结束,就返回
pub fn wait(exit_code: &mut i32) -> isize {
    loop {
        match sys_waitpid(-1, exit_code as *mut _) {
            -2 => {
                yield_();
            }
            // -1 or a real pid
            exit_pid => return exit_pid,
        }
    }
}

/// 必须等到指定pid的子进程结束,才返回
pub fn waitpid(pid: usize, exit_code: &mut i32) -> isize {
    loop {
        match sys_waitpid(pid as isize, exit_code as *mut _) {
            -2 => {
                yield_();
            }
            // -1 or a real pid
            exit_pid => return exit_pid,
        }
    }
}

pub fn waitpid_nb(pid: usize, exit_code: &mut i32) -> isize {
    sys_waitpid(pid as isize, exit_code as *mut _)
}

pub fn sleep(period_ms: usize) {
    let start = sys_get_time();
    while sys_get_time() < start + period_ms as isize {
        sys_yield();
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
/// 信号和对应handler
pub struct SignalAction {
    /// 信号处理函数的地址
    pub handler: usize,
    /// 信号掩码
    pub mask: SignalFlags,
}

impl Default for SignalAction {
    fn default() -> Self {
        Self {
            handler: 0,
            mask: SignalFlags::empty(),
        }
    }
}

pub const SIGDEF: i32 = 0; // Default signal handling
pub const SIGHUP: i32 = 1;
pub const SIGINT: i32 = 2;
pub const SIGQUIT: i32 = 3;
pub const SIGILL: i32 = 4;
pub const SIGTRAP: i32 = 5;
pub const SIGABRT: i32 = 6;
pub const SIGBUS: i32 = 7;
pub const SIGFPE: i32 = 8;
pub const SIGKILL: i32 = 9;
pub const SIGUSR1: i32 = 10;
pub const SIGSEGV: i32 = 11;
pub const SIGUSR2: i32 = 12;
pub const SIGPIPE: i32 = 13;
pub const SIGALRM: i32 = 14;
pub const SIGTERM: i32 = 15;
pub const SIGSTKFLT: i32 = 16;
pub const SIGCHLD: i32 = 17;
pub const SIGCONT: i32 = 18;
pub const SIGSTOP: i32 = 19;
pub const SIGTSTP: i32 = 20;
pub const SIGTTIN: i32 = 21;
pub const SIGTTOU: i32 = 22;
pub const SIGURG: i32 = 23;
pub const SIGXCPU: i32 = 24;
pub const SIGXFSZ: i32 = 25;
pub const SIGVTALRM: i32 = 26;
pub const SIGPROF: i32 = 27;
pub const SIGWINCH: i32 = 28;
pub const SIGIO: i32 = 29;
pub const SIGPWR: i32 = 30;
pub const SIGSYS: i32 = 31;

bitflags! {
    pub struct SignalFlags: i32 {
        /// Default signal handling
        const SIGDEF = 1;
        /// 挂起
        const SIGHUP = 1 << 1;
        /// 中断
        const SIGINT = 1 << 2;
        /// 退出
        const SIGQUIT = 1 << 3;
        /// 非法指令
        const SIGILL = 1 << 4;
        /// 断点或陷阱指令
        const SIGTRAP = 1 << 5;
        /// abort发出的信号
        const SIGABRT = 1 << 6;
        /// 非法内存访问
        const SIGBUS = 1 << 7;
        /// 浮点异常
        const SIGFPE = 1 << 8;
        /// kill信号
        const SIGKILL = 1 << 9;
        /// 用户信号1
        const SIGUSR1 = 1 << 10;
        /// 无效内存访问
        const SIGSEGV = 1 << 11;
        /// 用户信号2
        const SIGUSR2 = 1 << 12;
        /// 管道破损，没有读端的管道写数据
        const SIGPIPE = 1 << 13;
        /// alarm发出的信号
        const SIGALRM = 1 << 14;
        /// 终止信号
        const SIGTERM = 1 << 15;
        /// 栈溢出
        const SIGSTKFLT = 1 << 16;
        /// 子进程退出
        const SIGCHLD = 1 << 17;
        /// 进程继续
        const SIGCONT = 1 << 18;
        /// 进程停止
        const SIGSTOP = 1 << 19;
        /// 进程停止
        const SIGTSTP = 1 << 20;
        /// 进程停止，后台进程从终端读数据时
        const SIGTTIN = 1 << 21;
        /// 进程停止，后台进程想终端写数据时
        const SIGTTOU = 1 << 22;
        /// I/O有紧急数据到达当前进程
        const SIGURG = 1 << 23;
        /// 进程的CPU时间片到期
        const SIGXCPU = 1 << 24;
        /// 文件大小的超出上限
        const SIGXFSZ = 1 << 25;
        /// 虚拟时钟超时
        const SIGVTALRM = 1 << 26;
        /// profile时钟超时
        const SIGPROF = 1 << 27;
        /// 窗口大小改变
        const SIGWINCH = 1 << 28;
        /// I/O相关
        const SIGIO = 1 << 29;
        /// 关机
        const SIGPWR = 1 << 30;
        /// 系统调用异常
        const SIGSYS = 1 << 31;
    }
}

pub fn kill(pid: usize, signal: i32) -> isize {
    sys_kill(pid, signal)
}

pub fn sigaction(
    signum: i32,
    action: *const SignalAction,
    old_action: *const SignalAction,
) -> isize {
    sys_sigaction(signum, action, old_action)
}

pub fn sigprocmask(mask: u32) -> isize {
    sys_sigprocmask(mask)
}

pub fn sigreturn() -> isize {
    sys_sigreturn()
}

pub fn signal(signum: i32, handler: usize) -> isize {
    let mut new = SignalAction::default();
    let old = SignalAction::default();
    new.handler = handler;

    sigaction(signum, &new, &old)
}
