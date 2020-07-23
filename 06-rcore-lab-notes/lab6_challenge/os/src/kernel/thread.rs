//! 线程相关的内核功能

use super::*;

pub(super) fn sys_gettid() -> SyscallResult {
    SyscallResult::Proceed(PROCESSOR.lock().current_thread().id)
}

pub(super) fn sys_clone(context: &Context) -> SyscallResult {
    let thread = PROCESSOR.lock().current_thread().clone();
    let new_thread = thread.fork(&context).unwrap();
    new_thread.inner().context.unwrap().x[10] = 0;
    PROCESSOR.lock().add_thread(new_thread);
    SyscallResult::Proceed(thread.id)
}
