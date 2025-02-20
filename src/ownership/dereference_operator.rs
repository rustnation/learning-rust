pub fn master(show: bool) {
    if show {
        let brand = String::from("Kubernetes and Rust");
        let linked_brand = &brand;

        println!("Master brand: {}", *linked_brand);
    }
}
