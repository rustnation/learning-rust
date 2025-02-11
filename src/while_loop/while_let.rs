//! src/while_loop/while_let.rs
pub fn master(show: bool) {
    if show {
        println!("\n--- while let");

        simple_data();

        in_vector();
    }
}

fn simple_data() {
    let mut data = Some(7);

    while let Some(i) = data {
        println!("loop: {}", i);
        data = None;
    }

    println!("done");
}

fn in_vector() {
    println!("\n--- In vector");

    //let numbers = vec![1, 2, 3, 4, 5, 6, 7];
    let number_iter = [1, 2, 3, 4, 5, 6, 7].iter();
    /*while let Some(num) = number_iter.next() {
        println!("num: {:?}", num);
    }*/
    for num in number_iter {
        println!("num: {:?}", num);
    }
}
