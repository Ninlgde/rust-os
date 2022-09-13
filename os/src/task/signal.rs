//! 信号枚举
use bitflags::*;

/// 最大信号数量
pub const MAX_SIG: usize = 31;

bitflags! {
    /// 信号枚举
    pub struct SignalFlags: u32 {
        /// Default signal handling
        const SIGDEF = 1;
        /// 1
        const SIGHUP = 1 << 1;
        /// 1
        const SIGINT = 1 << 2;
        /// 1
        const SIGQUIT = 1 << 3;
        /// 1
        const SIGILL = 1 << 4;
        /// 1
        const SIGTRAP = 1 << 5;
        /// 1
        const SIGABRT = 1 << 6;
        /// 1
        const SIGBUS = 1 << 7;
        /// 1
        const SIGFPE = 1 << 8;
        /// 1
        const SIGKILL = 1 << 9;
        /// 1
        const SIGUSR1 = 1 << 10;
        /// 1
        const SIGSEGV = 1 << 11;
        /// 1
        const SIGUSR2 = 1 << 12;
        /// 1
        const SIGPIPE = 1 << 13;
        /// 1
        const SIGALRM = 1 << 14;
        /// 1
        const SIGTERM = 1 << 15;
        /// 1
        const SIGSTKFLT = 1 << 16;
        /// 1
        const SIGCHLD = 1 << 17;
        /// 1
        const SIGCONT = 1 << 18;
        /// 1
        const SIGSTOP = 1 << 19;
        /// 1
        const SIGTSTP = 1 << 20;
        /// 1
        const SIGTTIN = 1 << 21;
        /// 1
        const SIGTTOU = 1 << 22;
        /// 1
        const SIGURG = 1 << 23;
        /// 1
        const SIGXCPU = 1 << 24;
        /// 1
        const SIGXFSZ = 1 << 25;
        /// 1
        const SIGVTALRM = 1 << 26;
        /// 1
        const SIGPROF = 1 << 27;
        /// 1
        const SIGWINCH = 1 << 28;
        /// 1
        const SIGIO = 1 << 29;
        /// 1
        const SIGPWR = 1 << 30;
        /// 1
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
