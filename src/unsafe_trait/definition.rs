pub fn master(show: bool) {
    if show {
        println!("-- Unsafe Trait");
    }
}

/// # Safety
unsafe trait Foo {
    // methods go here
}

unsafe impl Foo for i32 {
    // method implementations go here
}
