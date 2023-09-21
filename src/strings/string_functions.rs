pub fn master(show: bool) {
    if show {
        println!("--- Working with string functions");

        let text = "I see the eigenvalue in thine eye";
        let (head, tail) = text.split_at(21);
        println!("head: {}", head);
        println!("tail: {}", tail);
    }
}
