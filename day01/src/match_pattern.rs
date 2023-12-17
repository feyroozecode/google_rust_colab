pub fn start() {
    f1('w');
    describe_point((1, 0));
}

fn f1(input: char) {
    match input {
        'q' => println!("Quitting"),
        'a' | 's' | 'w' | 'd' => println!("Moving player"),
        '0'..='9' => println!("Invalid move"),
        key if key.is_lowercase() => println!("Lowercase: {key}"),
        _ => println!("Invalid Something else"),
    }
}

// with tupple
fn describe_point(point: (i32, i32)) {
    match point {
        (0, _)            =>  println!("on Y axis"),
        (_, 0)            =>  println!("on X axis"),
        (x, _) if x < 0   =>  println!("left of Y axis"),
        (_, y)   if y < 0 =>  println!("Below X axix"),
        _                 =>  println!("First quadrant"),
    }
}
