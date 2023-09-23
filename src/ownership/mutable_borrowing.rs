pub fn master(show: bool) {
    if show {
        let mut one: i8 = 6;
        print(&mut one);
        println!("In master the value is: {}", one);
    }
}

fn print(value: &mut i8) {
    //value += 1; ToDo: this is not working
    println!("In function the value is: {}", value);
}
