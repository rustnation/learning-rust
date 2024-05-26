mod definition;
mod mutexes;

pub fn master(show: bool) {
    if show {
        // Array Definition
        definition::master(false);

        // Mutex in Practice
        mutexes::master(false);
    }
}
