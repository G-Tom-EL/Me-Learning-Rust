fn fibonacci(n: u32) -> u32 {
    if n < 2 {
        return n;
    } else {
        return fibonacci(n - 1) + fibonacci(n - 2);
    }
}


fn main() {
    let n :u32 = 20;
    println!("fib({n}) = {}", fibonacci(n));
}
