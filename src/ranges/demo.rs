pub fn master(show: bool) {
    if show {
        println!("\n--- Range Demo");

        for num in 1..4 {
            println!("{:?}", num);
        }

        for ch in 'a'..='f' {
            println!("{:?}", ch);
        }
    }
}
