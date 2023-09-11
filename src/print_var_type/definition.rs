use std::any::type_name;

fn type_of<T>(_: &T) -> &'static str {
    type_name::<T>()
}

pub fn master(show: bool) {
    if show {
        println!("\n--- Print variable type");

        let s = "Hello";
        println!("type of s: {:?}", type_of(&s));

        let  mut chaos = [3, 5, 4, 1, 2, 6, 7];
        println!("type of chaos: {:?}", type_of(&chaos));

        chaos.sort();
        println!("type of chaos: {:?}", type_of(&chaos));
    }
}