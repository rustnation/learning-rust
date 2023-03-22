pub fn shadowing() {
    let x = 7;
    println!("--- Shadowing ---");
    {
        let x = x *2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}");
}