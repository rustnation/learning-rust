pub fn master(show: bool) {
    if show {
        println!("--- String Cloning Heap ---");
        let s1 = String::from("hallo");
        let s2 = s1.clone(); // use of clone method

        println!("s1 = {s1}, s2 = {s2}");
    }
}
