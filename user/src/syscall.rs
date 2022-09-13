use crate::SignalAction;
use core::arch::asm;

const SYSCALL_LS: usize = 22;

const SYSCALL_DUP: usize = 24;
const SYSCALL_OPEN: usize = 56;
const SYSCALL_CLOSE: usize = 57;
const SYSCALL_PIPE: usize = 59;
const SYSCALL_READ: usize = 63;
const SYSCALL_WRITE: usize = 64;
const SYSCALL_EXIT: usize = 93;
const SYSCALL_YIELD: usize = 124;
const SYSCALL_KILL: usize = 129;
const SYSCALL_SIGACTION: usize = 134;
const SYSCALL_SIGPROCMASK: usize = 135;
const SYSCALL_SIGRETURN: usize = 139;
const SYSCALL_GET_TIME: usize = 169;
const SYSCALL_GETPID: usize = 172;
const SYSCALL_FORK: usize = 220;
const SYSCALL_EXEC: usize = 221;
const SYSCALL_WAITPID: usize = 260;

#[inline(always)]
fn syscall(id: usize, args: [usize; 3]) -> isize {
    let mut ret: isize;
    unsafe {
        asm!(
        "ecall",
        inlateout("x10") args[0] => ret,
        in("x11") args[1],
        in("x12") args[2],
        in("x17") id
        );
    }
    ret
}

/// 功能: 显示某个路径下的所有文件
/// 参数: `path` 待显示所有文件的路径
/// 返回值: 暂无返回值
/// syscall ID: 22(瞎编的)
pub fn sys_ls(path: &str) -> isize {
    syscall(SYSCALL_LS, [path.as_ptr() as usize, 0, 0])
}

/// 功能: 将进程中一个已经打开的文件复制一份并分配到一个新的文件描述符中。
/// 参数: `fd` 表示进程中一个已经打开的文件的文件描述符。
/// 返回值: 如果出现了错误则返回 -1，否则能够访问已打开文件的新文件描述符。
/// 可能的错误原因是: 传入的 fd 并不对应一个合法的已打开文件。
/// syscall ID: 24
pub fn sys_dup(fd: usize) -> isize {
    syscall(SYSCALL_DUP, [fd as usize, 0, 0]) as isize
}

/// 功能: 打开一个常规文件，并返回可以访问它的文件描述符。
/// 参数: `path` 描述要打开的文件的文件名（简单起见，文件系统不需要支持目录，所有的文件都放在根目录 / 下），
///      `flags` 描述打开文件的标志，具体含义下面给出。
/// 返回值: 如果出现了错误则返回 -1，否则返回打开常规文件的文件描述符。可能的错误原因是: 文件不存在。
/// syscall ID: 56
pub fn sys_open(path: &str, flags: u32) -> isize {
    syscall(SYSCALL_OPEN, [path.as_ptr() as usize, flags as usize, 0])
}

/// 功能: 当前进程关闭一个文件。
/// 参数: `fd` 表示要关闭的文件的文件描述符。
/// 返回值: 如果成功关闭则返回 0 ，否则返回 -1 。可能的出错原因: 传入的文件描述符并不对应一个打开的文件。
pub fn sys_close(fd: usize) -> isize {
    syscall(SYSCALL_CLOSE, [fd as usize, 0, 0])
}

/// 功能: 为当前进程打开一个管道。
/// 参数: `pipe` 表示应用地址空间中的一个长度为 2 的 usize 数组的起始地址，内核需要按顺序将管道读端
///             和写端的文件描述符写入到数组中。
/// 返回值: 如果出现了错误则返回 -1，否则返回 0 。可能的错误原因是: 传入的地址不合法。
/// syscall ID: 59
pub fn sys_pipe(pipe: &mut [usize]) -> isize {
    syscall(SYSCALL_PIPE, [pipe.as_mut_ptr() as usize, 0, 0])
}

/// 功能: 从文件中读取一段内容到缓冲区。
/// 参数: `fd` 是待读取文件的文件描述符，
///      `buffer` 切片 则给出缓冲区。
/// 返回值: 如果出现了错误则返回 -1，否则返回实际读到的字节数。
/// syscall ID: 63
pub fn sys_read(fd: usize, buffer: &mut [u8]) -> isize {
    syscall(
        SYSCALL_READ,
        [fd, buffer.as_mut_ptr() as usize, buffer.len()],
    )
}

/// 功能: 将内存中缓冲区中的数据写入文件。
/// 参数: `fd` 表示待写入文件的文件描述符；
///      `buffer` 表示内存中缓冲区的起始地址；
/// 返回值: 返回成功写入的长度。
/// syscall ID: 64
pub fn sys_write(fd: usize, buffer: &[u8]) -> isize {
    syscall(SYSCALL_WRITE, [fd, buffer.as_ptr() as usize, buffer.len()])
}

/// 功能: 退出应用程序并将返回值告知批处理系统。
/// 参数: `xstate` 表示应用程序的返回值。
/// 返回值: 该系统调用不应该返回。
/// syscall ID: 93
pub fn sys_exit(xstate: i32) -> isize {
    syscall(SYSCALL_EXIT, [xstate as usize, 0, 0])
}

/// 功能: 当前进程让出cpu
/// 返回值: 0
/// syscall ID: 124
pub fn sys_yield() -> isize {
    syscall(SYSCALL_YIELD, [0; 3])
}

/// 功能: 获取时间
/// 返回值: 当前时间
/// syscall ID: 169
pub fn sys_get_time() -> isize {
    syscall(SYSCALL_GET_TIME, [0; 3])
}

/// 功能: 获取process id
/// 返回值: process id
/// syscall ID: 172
pub fn sys_getpid() -> isize {
    syscall(SYSCALL_GETPID, [0, 0, 0])
}

/// 功能: 当前进程 fork 出来一个子进程。
/// 返回值: 对于子进程返回 0，对于当前进程则返回子进程的 PID 。
/// syscall ID: 220
pub fn sys_fork() -> isize {
    syscall(SYSCALL_FORK, [0, 0, 0])
}

/// 功能: 将当前进程的地址空间清空并加载一个特定的可执行文件，返回用户态后开始它的执行。
/// 参数: `path` 给出了要加载的可执行文件的名字；
/// 返回值: 如果出错的话（如找不到名字相符的可执行文件）则返回 -1，否则不应该返回。
/// syscall ID: 221
pub fn sys_exec(path: &str, args: &[*const u8]) -> isize {
    syscall(
        SYSCALL_EXEC,
        [path.as_ptr() as usize, args.as_ptr() as usize, 0],
    )
}

/// 功能: 当前进程等待一个子进程变为僵尸进程，回收其全部资源并收集其返回值。
/// 参数: `pid` 表示要等待的子进程的进程 ID，如果为 -1 的话表示等待任意一个子进程；
///      `exit_code` 表示保存子进程返回值的地址，如果这个地址为 0 的话表示不必保存。
/// 返回值: 如果要等待的子进程不存在则返回 -1；否则如果要等待的子进程均未结束则返回 -2；
///        否则返回结束的子进程的进程 ID。
/// syscall ID: 260
pub fn sys_waitpid(pid: isize, exit_code: *mut i32) -> isize {
    syscall(SYSCALL_WAITPID, [pid as usize, exit_code as usize, 0])
}

/// 功能: 向指定进程发送信号
/// 参数: `pid` 表示接收信号的进程
///      `signal` 表示要发送的信号
/// 返回值: 0成功 -1失败
/// syscall ID: 129
pub fn sys_kill(pid: usize, signal: i32) -> isize {
    syscall(SYSCALL_KILL, [pid, signal as usize, 0])
}

/// 功能: 替换进程对应信号的action
/// 参数: `signum` 表示信号
///      `action` 表示新的信号和action
///      `old_action` 表示被替换的信号和action
/// 返回值: 0成功 -1失败
/// syscall ID: 134
pub fn sys_sigaction(
    signum: i32,
    action: *const SignalAction,
    old_action: *const SignalAction,
) -> isize {
    syscall(
        SYSCALL_SIGACTION,
        [signum as usize, action as usize, old_action as usize],
    )
}

/// 功能: 向指定进程发送信号
/// 参数: `mask` 表示接收信号的进程
/// 返回值: -1失败 其他值表示`old_mask`
/// syscall ID: 135
pub fn sys_sigprocmask(mask: u32) -> isize {
    syscall(SYSCALL_SIGPROCMASK, [mask as usize, 0, 0])
}

/// 功能: 在信号处理后恢复继续执行
/// 返回值: 0成功 -1失败
/// syscall ID: 139
pub fn sys_sigreturn() -> isize {
    syscall(SYSCALL_SIGRETURN, [0, 0, 0])
}
