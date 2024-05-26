pub fn master(show: bool) {
    if show {
        println!("--- Mutable Reference");

        let mut my_number = 8;
        let num_ref = &mut my_number;

        *num_ref += 1;
        println!("my_number: {}", my_number);

        let second_number = 800;
        let triple_reference = &&&second_number;
        println!("Are they equal? {}", second_number == ***triple_reference);
    }
}
