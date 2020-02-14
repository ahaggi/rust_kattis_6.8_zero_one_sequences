use std::io;
use std::io::prelude::*;

mod all_the_solutions;
fn main() {
    all_the_solutions::run()
    // let  mult = |a: u32, b: u32| -> u32 {
    //     let mut a:u64 = a as u64;
    //     let mut b:u64 = b as u64;
         
    //     ((a * b) % 1000000007) as u32
    // };
    // let mut look_up_table: [u32; 2] = [73741817, 147483634];
    // let mut latest_k = 31;
    // let mut get_2_pow_k = |_k: u32| {
    //     if _k < 30 {
    //         return 1 << _k;
    //     } else {
    //         if _k == latest_k - 1 {
    //             return look_up_table[0];
    //         }

    //         if _k == latest_k {
    //             return look_up_table[1];
    //         }

    //         look_up_table[0] = look_up_table[1];
    //         look_up_table[1] = take_exp_mod(_k);
    //         latest_k += 1;
    //         return look_up_table[1]
    //     }
    // };

    // let mut nr_of_ones: u32 = 0;
    // let mut res: u32 = 0;
    // let mut k: u32 = 0;
    // let mut secret_ingrediente = 0;

    // let mut _2_pow_k = 1;
    
    // let stdin = io::stdin();
    // let mut iter = stdin.lock().bytes();
    
    // while let Some(val) = iter.next() {
    //     match val {
    //         Ok(c) => {
    //             if c == b'1' {
    //                 nr_of_ones += 1;
    //             } else if c == b'0' {
    //                 res =  (res + mult(_2_pow_k , nr_of_ones) + secret_ingrediente) % 1000000007;
    //             } else if c == b'?' {
    //                 res =  ((res * 2) + mult(_2_pow_k , nr_of_ones) + secret_ingrediente) % 1000000007;
    //                 k += 1;
    //                 _2_pow_k = get_2_pow_k(k);
    //                 secret_ingrediente = if k <= 0 {
    //                     0
    //                 } else {
    //                     mult((k) , get_2_pow_k(k - 1))
    //                 };
    //             } else if c == b'\n' {
    //                 break;
    //             }
    //         }
    //         _ => break,
    //     };
    // }
    
    // println!("{:?}", res);
}





