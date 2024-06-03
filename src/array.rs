pub fn run() {
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    let str = String::from("Hello World!");
    println!(
        "size of array {} {}",
        std::mem::size_of_val(&arr),
        str.len()
    );

    let slice = &arr[0..2];
    println!("{}", slice[0]);

    let arr_of_tuples: [(&str, i32); 2] = [("foo", 1), ("bar", 2)];
    println!("{}", arr_of_tuples[0].0);
}
