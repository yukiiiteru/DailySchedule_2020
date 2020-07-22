//! 系统调用

pub const STDIN: usize = 0;
pub const STDOUT: usize = 1;

const SYSCALL_OPENAT: usize = 56;
const SYSCALL_CLOSE: usize = 57;
const SYSCALL_READ: usize = 63;
const SYSCALL_WRITE: usize = 64;
const SYSCALL_EXIT: usize = 93;
const SYSCALL_GETTID: usize = 178;
const SYSCALL_CLONE: usize = 220;

/// 将参数放在对应寄存器中，并执行 `ecall`
fn syscall(id: usize, arg0: usize, arg1: usize, arg2: usize, arg3: usize, arg4: usize, arg5: usize) -> isize {
    // 返回值
    let mut ret;
    unsafe {
        llvm_asm!("ecall"
            : "={x10}" (ret)
            : "{x10}" (arg0), "{x11}" (arg1), "{x12}" (arg2), "{x13}" (arg3), "{x14}" (arg4), "{x15}" (arg5), "{x17}" (id)
            : "memory"      // 如果汇编可能改变内存，则需要加入 memory 选项
            : "volatile"); // 防止编译器做激进的优化（如调换指令顺序等破坏 SBI 调用行为的优化）
    }
    ret
}

/// 打开文件
pub fn sys_open(path: &str, flags: usize, mode: usize) -> isize {
    sys_openat(
        AT_FDCWD,
        path.as_bytes().as_ptr() as *const u8,
        flags,
        mode
    )
}

/// 打开文件
pub fn sys_openat(dir_fd: usize, path: *const u8, flags: usize, mode: usize) -> isize {
    syscall(
        SYSCALL_OPENAT,
        dir_fd,
        path as usize,
        flags,
        mode,
        0, 0
    )
}

pub fn sys_close(fd: usize) -> isize {
    syscall(SYSCALL_CLOSE, fd, 0, 0, 0, 0, 0)
}

/// 读取字符
pub fn sys_read(fd: usize, buffer: &mut [u8]) -> isize {
    loop {
        let ret = syscall(
            SYSCALL_READ,
            fd,
            buffer as *const [u8] as *const u8 as usize,
            buffer.len(),
            0, 0, 0
        );
        if ret > 0 {
            return ret;
        }
    }
}

/// 打印字符串
pub fn sys_write(fd: usize, buffer: &[u8]) -> isize {
    syscall(
        SYSCALL_WRITE,
        fd,
        buffer as *const [u8] as *const u8 as usize,
        buffer.len(),
        0, 0, 0
    )
}

/// 退出并返回数值
pub fn sys_exit(code: isize) -> ! {
    syscall(SYSCALL_EXIT, code as usize, 0, 0, 0, 0, 0);
    unreachable!()
}

/// 获得线程ID
pub fn sys_gettid() -> isize {
    syscall(SYSCALL_GETTID, 0, 0, 0, 0, 0, 0)
}

/// 克隆线程
pub fn sys_clone() -> isize {
    syscall(SYSCALL_CLONE, 0, 0, 0, 0, 0, 0)
}

/// Pathname is interpreted relative to the current working directory(CWD)
const AT_FDCWD: usize = -100isize as usize;
