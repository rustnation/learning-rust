pub fn master(show: bool) {
    if show {
        println!("--- Boolean Type ---");

        let t = true;
        println!("value of t is: {t}");

        let f = false;
        println!("value of f is: {f}");

        let age = 50;
        if age <= 20 {
            println!("the person is young");
        } else if age > 20 || age <= 35 {
            println!("the person is in the middle age");
        } else if age > 35 || age <= 50 {
            println!("the person is mature");
        } else if age > 50 {
            println!("the person is getting old");
        }
    }
}
