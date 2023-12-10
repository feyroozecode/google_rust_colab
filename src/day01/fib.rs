// with match
fn fib(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fib( n -1 )+ fib( n - 2)
    }
}

// with if else
fn fib2(n: u32) -> u32 {
    if n == 0 {
       return 0; 
    }
    else if n == 1 {
        return 1;
    }
    else {
        return fib2( n - 1 ) + fib2( n -2 );
    }
}

// gived solution
fn fib3(n: u32) -> u32 {
    if n <= 2 {
        return 1;
    } else {
        return fib(n - 1) + fib(n - 2);
    }
}

fn main() {
    let n = 5;
    println!("fib(n) = {}", fib2(n));
}
