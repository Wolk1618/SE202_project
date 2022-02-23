use clap::Parser;


#[derive(clap::Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// rank of fibo to compute
    #[clap(short = 'c', long = "count")]
    count: u32,
}

fn main() {
    let args = Args::parse();
    for i in 0..=args.count {
        let val: Option<u32> = fibo(i);
        match val {
            None => return,
            _    => println!("fibo({}) = {}", i, val.unwrap())
        }
        
    }
}

fn fibo(n: u32) -> Option<u32> {
    let mut new_val: Option<u32> = Some(1);
    let mut former_val: Option<u32> = Some(0);
    let mut inter: Option<u32>;
    if n == 0 {
        return Some(0);
    } else if n == 1 {
        return Some(1);
    }
    for _ in 1..n {
        inter = new_val;
        new_val = new_val.unwrap().checked_add(former_val.unwrap());
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