pub fn run() {
    let str: Box<str> = "Hello World".into();
    let str2: &str = "Hello World";
    let str3 = &str[0..2];

    let mut s = "Hello World".to_string();
    s.push_str(" o_O");
    s.push('!');
    s += "asdasd"; // we can concat String only with string slices String -> &str: as_str or &
    s.replace("o_O", "");

    let first_letter = &str[0..1]; // we need a string slice
    let first_letter2 = &str2[0..1]; // we need a string slice

    // str2.chars(): &str -> iterator
}
