pub fn run() {
    let _unused = "unused variable"; // prepended with _
    let foo = 1; // statement
    let mut bar = foo;
    println!("{} {}", foo, bar);
    bar = 2;

    println!("{}", bar);

    let (a, ..) = (4, 5); // destructuring

    shadowing();
    move1();
}

fn shadowing() {
    let x = 1;
    let x = "Shadowed x";
    let y = {
        // inside - expression as it produces the value
        let y = "Inner y in Scope";
        println!("Print y from inner {}", y);
        y // return y from scope
    }; // statement
    println!("x: {}, y: {}", x, y);
} // statement

fn move1() {
    let str = String::from("Hello"); // pointer
    let str2 = str; // str moved to str2, str - dropped
    let _str3 = str2.clone(); // we clone and can use str2 and str3 now

    println!("");
}
