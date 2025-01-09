pub fn master(show: bool) {
    if show {
        println!("-- Compiler Directives --");

        #[allow(unused_variables)]
        let message = "unused variable";

        #[allow(unused_variables)]
        let expected_message = "other unused variable";
    }
}
