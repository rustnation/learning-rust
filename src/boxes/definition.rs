pub fn master(show: bool) {
    if show {
        println!("-- Using Box to Store Data on Heap");
        store_i32_on_heap();
    }
}

fn store_i32_on_heap() {
    let b = Box::new(5);
    println!("b = {b}");
}
