use crate::print_title;
mod reading_csv_file;

pub fn master(show: bool) {
    if show {
        print_title("Machine Learning");

        let _ = reading_csv_file::master(false);
    }
}
