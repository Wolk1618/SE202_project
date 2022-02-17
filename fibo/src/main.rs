fn main() {
    for i in 0..=42 {
        println!("fibo({}) = {}", i, fibo(i))
    }
}

fn fibo(n: u32) -> u32 {
    let mut new_val = 1;
    let mut former_val = 0;
    let mut inter;
    for _ in 1..=n {
        inter = new_val;
        new_val = new_val + former_val;
        former_val = inter;
    }
    return new_val;

}

/*
fn fibo(n: u32) -> u32 {
    if n==0 {
        0
    } else if n==1 {
        1
    } else {
        fibo(n-1) + fibo(n-2)
    }
}*/