pub fn string_clone_heap() {
    println!("--- String Cloning Heap ---");
    let s1 = String::from("hallo");
    let s2 = s1.clone(); // use of clone method

    println!("s1 = {s1}, s2 = {s2}");
}
