use std::cmp::Ordering;
use std::ffi::c_int;
use std::io;

fn  calculateFibonacciNumber(n:usize) -> Vec<u64>{
    let mut vResult:Vec<u64>=Vec::new();
    if n < 1 {
        println!("{} should be >0" ,n);
    } else if n==1 {vResult.push(0)}
    else if n >=2{
        vResult.push(0);
        vResult.push(1);
        let mut sum = 0;
      for i in 1..n-1{
            sum = &vResult[i]+&vResult[i-1];
            vResult.push(sum);
        }
    }
    return vResult;
  }

fn main() {
    let n :usize=10;
    let fibo=calculateFibonacciNumber(n);
    for x in fibo.iter() {
        print!("{} ", x);
    }

}
