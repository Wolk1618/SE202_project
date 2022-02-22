fn main() {
    for i in 0..=50 {
        println!("fibo({}) = {}", i, fibo(i))
    }
}

fn fibo(n: u32) -> u32 {
    let mut new_val: u32 = 1;
    let mut former_val: u32 = 0;
    let mut inter: u32;
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    }
    for _ in 1..n {
        inter = new_val;
        new_val = new_val.saturating_add(former_val);
        former_val = inter;
    }
    new_val

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