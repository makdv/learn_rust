pub fn run() {
    let x = Box::new(6);
    let mut y = x; // takes ownership
    *y = 4; // change value

    let _z = take_ownership(y);
}

fn take_ownership(i: Box<i32>) -> Box<i32> {
    println!("{}", i);
    i
}

// borrowing: we can have
// 1 mutable reference in the same scope or
// any number of immutable references
// references must be always valid
#[warn(dead_code)]
fn borrowing() {
    let mut str = String::from("Test");
    let str1 = &str;
    let str2 = &str;
    // let str3 = &mut str; // -  would be a big problem

    println!("{}, {}", str1, str2);

    // str1,str2 - not used anymore

    let str3 = &mut str; // - no problem now
    println!("{}", str3);
}

fn dangle_use() {
    let hold_dnagle = dangle();
}
fn dangle() -> String {
    let str = String::from("Test");

    // &str - Bad - str will be deleted
    str
}
