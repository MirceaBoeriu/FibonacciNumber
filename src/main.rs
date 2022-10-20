use std::cmp::Ordering;
use std::ffi::c_int;
use std::io;

fn  calculateFibonacciNumber(n:i32) -> Vec<i64>{
    let mut vResult:Vec<i64>=Vec::new();
    if n < 1 {
        println!("{} should be >0" ,n);
    } else {
        let mut sum = 0;
        let mut prev2 = 0;
        let mut prev1 = 1;
       // print!("{sum}");
    vResult.push(sum);
        for i in 0..n{

                sum = prev1 + prev2;
                prev2 = prev1;
                prev1 = sum;
                vResult.push(sum);
            //print!(", {sum}");
            }
    }
    return vResult;

  }

/*fn  calculateFibonacciNumber(n:i32)-> Vec<u64> {
    let mut vResult : Vec<u64>=(0..=1).collect();



       match n {
       // Match several values
         1=> vResult.pop(),
        // Handle the rest of cases
       _ => {
           let mut sum = 0;
           let mut prev2 = 0;
           let mut prev1 = 1;
           for _i in 0..=n {
               print!("{} ",_i);
               /*sum = prev1 + prev2;
               prev2 = prev1;
               prev1 = sum;*/
               //vResult.push(sum);
           }



                    },

         // TODO ^ Try commenting out this catch-all arm
     }

return vResult;
}*/

fn main() {
    let n :i32=50;
  //  calculateFibonacciNumber(n);

   let fibo=calculateFibonacciNumber(n);
 //println!("{}",fibo[1]);
    for x in fibo.iter() {
        print!("{} ", x);
    }

}
