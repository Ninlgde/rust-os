//! impl actions for a signal
use crate::task::{SignalFlags, MAX_SIG};

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
            mask: SignalFlags::from_bits(40).unwrap(),
        }
    }
}

#[derive(Clone)]
/// Actions for a signal
pub struct SignalActions {
    pub table: [SignalAction; MAX_SIG + 1],
}

impl Default for SignalActions {
    fn default() -> Self {
        Self {
            table: [SignalAction::default(); MAX_SIG + 1],
        }
    }
}
