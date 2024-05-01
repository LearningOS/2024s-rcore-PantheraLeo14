//!sys config

#[allow(unused)]


/// write syscall
pub const SYSCALL_WRITE: usize = 64;
/// exit syscall
pub const SYSCALL_EXIT: usize = 93;
/// yield syscall
pub const SYSCALL_YIELD: usize = 124;
/// gettime syscall
pub const SYSCALL_GET_TIME: usize = 169;
/// taskinfo syscall
pub const SYSCALL_TASK_INFO: usize = 410;