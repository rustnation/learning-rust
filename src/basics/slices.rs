pub fn master() {
    println!("--- Slices ---");
    let slice_array: [i32; 100] = [0; 100];
    println!("slice: {:?}", &slice_array[5 .. 8]);
}