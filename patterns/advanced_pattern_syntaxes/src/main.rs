struct Point {
    x: i32,
    y: i32,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn destructure_pattern_matching() {
    let p = Point { x: 0, y: 7 };

    let Point { x: a, y: b } = p; // destructuring the patterns with mapping x to a and y to b {this is optional BTW, you can keep the variable destructuring as it is}
    assert_eq!(0, a);
    assert_eq!(7, b);

    // ------------------------------------------------
    // Custom pattern destructuring example
    // ------------------------------------------------

    // Structs //

    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis at ({}, {})", x, y),
    }

    // Enums //

    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => println!("The Quit variant has no data to destructure."),
        Message::Move { x, y } => println!("x: {} y: {}", x, y),
        Message::Write(text) => println!("Text: {}", text),
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red: {}, green: {}, blue: {}", r, g, b)
        }
    }
}

fn main() {
    // ------------------------------------------------
    // Matching the literals
    // ------------------------------------------------

    let x = 1;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // ------------------------------------------------
    // Named variables matching with printing
    // ------------------------------------------------

    let x = Some(10);

    match x {
        Some(10) => println!("ten"),
        Some(i) => println!("Value {}", i),
        _ => println!("none"),
    }

    // ------------------------------------------------
    // Multi pattern matching (such as using & or |)
    // ------------------------------------------------

    let x = 10;

    match x {
        1 | 2 | 3 | 4 | 5 => println!("one through five"),
        _ => println!("something else"),
    }

    // ------------------------------------------------
    // Adding range patterns matching
    // ------------------------------------------------

    let x = 6;

    match x {
        1..=10 => println!("one through ten"),
        _ => println!("something else"),
    }

    let x = 'c';

    match x {
        'a'..='j' => println!("early letter"),
        'k'..='z' => println!("late letter"),
        _ => println!("This is something else"),
    }

    destructure_pattern_matching();
}