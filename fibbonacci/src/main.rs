fn main() {
    println!("{}", fib(5));
}

fn fib(num: u32) -> u32 {
    if num == 0 {
        return 0;
    }
    if num == 1 {
        return 1;
    }
    return fib(num - 1) + fib(num - 2);
}
