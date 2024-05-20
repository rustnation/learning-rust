use crate::print_title;
pub mod raw_syscall;

pub fn master(show: bool) {
    if show {
        print_title("System Calls");

        raw_syscall::master(false);
    }
}
