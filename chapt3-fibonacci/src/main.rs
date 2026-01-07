fn fibonacci(n: i32) -> Vec<i32> {
    if n == 0 {
        return [0].to_vec();
    } else {
        let mut a = 0;
        let mut b = 1;
        let mut fibo_array: Vec<i32> = [a, b].to_vec();
        for _ in 1..n + 1 {
            let c = a + b;
            a = b;
            b = c;
            fibo_array.push(c);
        }
        return fibo_array;
    }
}

fn main() {
    let n = 10;
    let vector = fibonacci(n);
    println!("{vector:?}");
}
