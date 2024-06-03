enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

pub fn run() {
    let n = 5;
    guess(n);
    for_loop(n);
    while_loop();
    infinite_loop();
    match_condition(Coin::Penny);
}

fn guess(n: i32) {
    let x = if n < 0 {
        println!("negative: {}", n);
    } else if n > 0 {
        println!("positive: {}", n);
    } else {
        println!("zero: {}", n);
    };

    x
}

fn for_loop(n: i32) {
    for i in 1..n {
        if i == 100 {
            println!("n = 100");
        }
    }

    let nums = [1, 2, 3];
    for i in nums {
        println!("{}", i);
    }

    for (i, v) in nums.iter().enumerate() {
        println!("index:{}, value:{}", i, v);
    }
}

fn while_loop() {
    let mut n = 1;

    while n < 10 {
        n += 1;
    }

    println!("while: {}", n);
}

fn infinite_loop() {
    let mut count = 1;

    let result = loop {
        count += 1;

        if count == 10 {
            break 100;
        }
    };

    println!("loop: {}", result);

    // we can use 'outer: / 'inner: goto labels before loop
}

fn match_condition(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
