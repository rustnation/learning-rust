pub fn master(show: bool) {
    if show {
        println!("-- Custom Smart Pointer");
        definition();
    }
}

fn definition() {
    let _c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let _d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created. (Called before end of main)");
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!(
            "Dropping CustomSmartPointer with data `{}`!. (Called at the end of main)",
            self.data
        );
    }
}
