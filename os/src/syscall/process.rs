//! Process management syscalls
use crate::{
    config::MAX_SYSCALL_NUM,
    task::{exit_current_and_run_next, suspend_current_and_run_next, TaskStatus, set_task_info},
    timer::get_time_us,
};
///
#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    ///
    pub sec: usize,
    ///
    pub usec: usize,
}

/// Task information
#[allow(dead_code)]
pub struct TaskInfo {
    /// Task status in it's life cycle
    status: TaskStatus,
    /// The numbers of syscall called by task
    syscall_times: [u32; MAX_SYSCALL_NUM],
    /// Total running time of task
    time: usize,
}
///
impl TaskInfo {
    /// Getter for status
    pub fn get_status(&self) -> &TaskStatus {
        &self.status
    }

    /// Setter for status
    pub fn set_status(&mut self, new_status: TaskStatus) {
        self.status = new_status;
    }

    /// Getter for syscall_times
    pub fn get_syscall_times(&self) -> &[u32; MAX_SYSCALL_NUM] {
        &self.syscall_times
    }
    ///
    pub fn set_syscall_times(&mut self, new_values: &[u32; MAX_SYSCALL_NUM]) {
        self.syscall_times.copy_from_slice(new_values);
    }

    /// Method to increment a syscall_time by index
    pub fn increment_syscall_time(&mut self, index: usize) {
        if index < MAX_SYSCALL_NUM {
            self.syscall_times[index] += 1;
        } else {
            panic!("Index out of bounds");
        }
    }

    /// Getter for time
    pub fn get_time(&self) -> usize {
        self.time
    }

    /// Setter for time
    pub fn set_time(&mut self, new_time: usize) {
        self.time = new_time;
    }
}

/// task exits and submit an exit code
pub fn sys_exit(exit_code: i32) -> ! {
    trace!("[kernel] Application exited with code {}", exit_code);
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    suspend_current_and_run_next();
    0
}

/// get time with second and microsecond
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    let us = get_time_us();
    unsafe {
        *ts = TimeVal {
            sec: us / 1_000_000,
            usec: us % 1_000_000,
        };
    }
    0
}

/// YOUR JOB: Finish sys_task_info to pass testcases
pub fn sys_task_info(_ti: *mut TaskInfo) -> isize {
    set_task_info(_ti);

    0  // Return 0 for success

    // trace!("kernel: sys_task_info");
    // -1
}
