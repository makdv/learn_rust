pub fn run() {
    let _v: () = (); // unit type, 0 bytes - nothing
    let my_bool = true; // bool, 1 byte
    let my_int = -1; // i32 - default, i8, i16, i32, i64, i128
    let my_uint = 1; // u8, u16, u32, u64, u128
    let my_float: f64 = 1.1; // f64 - default, f32, f64;
    let my_float_ex1 = 2f64; // типы можно прописывать таким образом
    let my_float_ex2 = 3_f64; // и таким
    let my_int_ex1 = 2 as i8; // и таким
    let my_usize: isize = 1; // usize, isize
    let my_char = 'C'; // 4 bytes
    let my_tuple: (i32, &str, char) = (1, "Hello World", 'S'); // Tuple
    let my_arr: [i32; 3] = [1, 2, 3]; // Array;
    let my_str_slice: &str = "Hello World"; // String slice / string literal

    let my_str = my_str_slice.to_string(); // String - stored on a Heap, == String.from("Hello World")
    // my_str variable: pointer to the value + len + capacity, 
    // every field is usize = 8 bytes on 64 architecture
    // so it takes 3 * 8 bytes for my_str variable
}
