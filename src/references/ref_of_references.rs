pub fn master(show: bool) {
    if show {
        println!("--- References of References");

        let my_number = 15;
        let single_reference = &my_number;
        let double_reference = &single_reference;

        println!("double_references: {:?}", **double_reference);
    }
}
