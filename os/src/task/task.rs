//! Types related to task management

use crate::config::{kernel_stack_position, TRAP_CONTEXT};
use crate::mm::{KERNEL_SPACE, MapPermission, MemorySet, PhysPageNum, VirtAddr};
use crate::trap::{trap_handler, TrapContext};
use super::TaskContext;

/// 任务状态
#[derive(Copy, Clone, PartialEq)]
pub enum TaskStatus {
    /// 未初始化
    UnInit,
    /// 准备运行
    Ready,
    /// 正在运行
    Running,
    /// 已退出
    Exited,
}

pub struct TaskControlBlock {
    /// 任务运行状态
    pub task_status: TaskStatus,
    /// 任务的上下文
    pub task_cx: TaskContext,
    /// 应用的地址空间
    pub memory_set: MemorySet,
    /// Trap 上下文被实际存放在物理页帧的物理页号
    pub trap_cx_ppn: PhysPageNum,
    /// 应用数据的大小
    pub base_size: usize,
}

/// implement
impl TaskControlBlock {
    pub fn get_trap_cx(&self) -> &'static mut TrapContext {
        self.trap_cx_ppn.get_mut()
    }
    pub fn get_user_token(&self) -> usize {
        self.memory_set.token()
    }
    pub fn new(elf_data: &[u8], app_id: usize) -> Self {
        // 解析传入的 ELF 格式数据构造应用的地址空间
        let (memory_set, user_sp, entry_point) = MemorySet::from_elf(elf_data);
        // 获取trap context 的物理页帧
        let trap_cx_ppn = memory_set
            .translate(VirtAddr::from(TRAP_CONTEXT).into())
            .unwrap()
            .ppn();
        // 标记任务状态为ready
        let task_status = TaskStatus::Ready;
        // 根据app_id 获取 app在kernel stack中的位置,并插入到内核空间中
        let (kernel_stack_bottom, kernel_stack_top) = kernel_stack_position(app_id);
        KERNEL_SPACE.exclusive_access().insert_framed_area(
            kernel_stack_bottom.into(),
            kernel_stack_top.into(),
            MapPermission::R | MapPermission::W,
        );
        let task_control_block = Self {
            task_status,
            task_cx: TaskContext::goto_trap_return(kernel_stack_top),
            memory_set,
            trap_cx_ppn,
            base_size: user_sp,
        };
        // prepare TrapContext in user space
        let trap_cx = task_control_block.get_trap_cx();
        *trap_cx = TrapContext::app_init_context(
            entry_point,
            user_sp,
            KERNEL_SPACE.exclusive_access().token(),
            kernel_stack_top,
            trap_handler as usize,
        );
        task_control_block
    }
}