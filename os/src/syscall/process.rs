//! Process management syscalls
use crate::config::{self, PAGE_SIZE};
use crate::mm::{translated_byte_buffer, write_to_byte_buffer, MapPermission};
use crate::task::{
    change_program_brk, current_user_token, exit_current_and_run_next, suspend_current_and_run_next,
};
use crate::timer::get_time_us;

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

/// task exits and submit an exit code
pub fn sys_exit(_exit_code: i32) -> ! {
    trace!("kernel: sys_exit");
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    suspend_current_and_run_next();
    0
}

/// YOUR JOB: get time with second and microsecond
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TimeVal`] is splitted by two pages ?
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    let us = get_time_us();
    let t = TimeVal {
        sec: us / 1_000_000,
        usec: us % 1_000_000,
    };
    unsafe {
        write_to_byte_buffer(
            current_user_token(),
            ts as *const u8,
            core::slice::from_raw_parts(
                &t as *const _ as *const u8,
                core::mem::size_of::<TimeVal>(),
            ),
        );
    }
    0
}

/// TODO: Finish sys_trace to pass testcases
/// HINT: You might reimplement it with virtual memory management.
pub fn sys_trace(trace_request: usize, id: usize, data: usize) -> isize {
    trace!("kernel: sys_trace");
    if id > config::MEMORY_END {
        return -1;
    }
    match (trace_request, id, data) {
        (0, id, _) => {
            let b = translated_byte_buffer(current_user_token(), id as *const u8, 1);
            if b.is_empty() {
                return -1;
            }
            b[0][0].into()
        }
        (1, id, data) => write_to_byte_buffer(current_user_token(), id as *const u8, &[data as u8]),
        (2, id, _) => crate::task::get_syscall_count(id) as isize,
        _ => -1,
    }
}

// YOUR JOB: Implement mmap.
pub fn sys_mmap(start: usize, len: usize, prot: usize) -> isize {
    trace!("kernel: sys_mmap");
    if start % PAGE_SIZE != 0 {
        return -1;
    }
    if prot == 0 || prot & !0x7 != 0 {
        return -1;
    }
    crate::task::save_frame(
        start.into(),
        (start + len).into(),
        MapPermission::from_bits((prot << 1) as u8).unwrap() | MapPermission::U,
    )
}

// YOUR JOB: Implement munmap.
pub fn sys_munmap(start: usize, len: usize) -> isize {
    trace!("kernel: sys_munmap");
    // unmmap(current_user_token(), start, len)
    if start % PAGE_SIZE != 0 || len % PAGE_SIZE != 0 {
        return -1;
    }
    crate::task::unmap_frame(start.into(), (start + len).into())
}
/// change data segment size
pub fn sys_sbrk(size: i32) -> isize {
    trace!("kernel: sys_sbrk");
    if let Some(old_brk) = change_program_brk(size) {
        old_brk as isize
    } else {
        -1
    }
}
