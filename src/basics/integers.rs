pub fn master() {
    println!("-- Integers");
    byte_literals();
}

fn byte_literals() {
    println!("-- Byte Literals");
    let x_ascii = b'x';
    println!("x in ASCII is: {}", x_ascii);
}
