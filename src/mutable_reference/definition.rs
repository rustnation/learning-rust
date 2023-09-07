// move the value behind a mutable reference
pub fn replace_with_84(s: &mut Box<i32>) {
    // this is not okay, as *s would be empty
    // let was = *s;
    // but this is:
    let was = std::mem::take(s);
    // so this is:
    *s = was;
    // we can exchange values behind &mut:
    let mut r = Box::new(84); // is r put in the heap?
    std::mem::swap(s, &mut r);
    assert_ne!(*r, 84);
}

pub fn master(show: bool) {
    if show {
        println!("Mutable Reference");
        let mut s = Box::new(42);
        println!("value of s: {}", s);
        replace_with_84(&mut s);
        println!("value of s: {}", s);
    }
}