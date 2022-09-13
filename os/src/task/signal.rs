//! 信号枚举
use bitflags::*;

/// 最大信号数量
pub const MAX_SIG: usize = 31;

bitflags! {
    /// 信号枚举
    pub struct SignalFlags: u32 {
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

impl SignalFlags {
    /// 信号检查
    pub fn check_error(&self) -> Option<(i32, &'static str)> {
        if self.contains(Self::SIGINT) {
            Some((-2, "Killed, SIGINT=2"))
        } else if self.contains(Self::SIGILL) {
            Some((-4, "Illegal Instruction, SIGILL=4"))
        } else if self.contains(Self::SIGABRT) {
            Some((-6, "Aborted, SIGABRT=6"))
        } else if self.contains(Self::SIGFPE) {
            Some((-8, "Erroneous Arithmetic Operation, SIGFPE=8"))
        } else if self.contains(Self::SIGKILL) {
            Some((-9, "Killed, SIGKILL=9"))
        } else if self.contains(Self::SIGSEGV) {
            Some((-11, "Segmentation Fault, SIGSEGV=11"))
        } else {
            //println!("[K] signalflags check_error  {:?}", self);
            None
        }
    }
}
