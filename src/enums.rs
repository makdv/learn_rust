enum SimpleEnum {
    Zero,
    One,
    Tow,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// internal
// enum Option<T> {
//     None,
//     Some(T),
// }

pub fn run() {
    println!("Enum: {}", SimpleEnum::Zero as u8);

    let msg: Message = Message::Move { x: 1, y: 2 };

    let six = option(Some(5));
    let none = option(None);

    if let Some(n) = six {
        println!("if let example: {}", n);
    }
}

fn option(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
