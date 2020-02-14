use rand::Rng;
use std::time::{Duration, Instant};

fn generate_test_text() -> String {
    let mut rng = rand::thread_rng();
    const CHARSET: &[u8] = b"01?";
    let len: usize = rng.gen_range(1, 50);
    let text: String = (0..len)
        .map(|_| {
            let idx = rng.gen_range(0, CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();
    println!("{:?}", text);
    text
}

pub fn run() {
    // let text = "?0?".to_string();
    let mut avg_zero = 0;
    let mut avg_zero_1 = 0;

    for i in 0..10000 {
        // let text = "1???111??1?0?0?010???0?1?110???";

        let text = "1???111??1?0?0?010???0?1?110????0101?00??01??1???111??1?0?0?010???0?1?110????0101?00??01??";
        let now = Instant::now();
        let zero = sol2(&text);
        let elapsed1 = now.elapsed().as_nanos();
        avg_zero += elapsed1;
        println!("zero {} The time elapsed is: {} Ns.", zero, elapsed1);

        let now = Instant::now();
        let zero_1 = sol_1_utilizing_look_up_table(&text);
        let elapsed2 = now.elapsed().as_nanos();
        avg_zero_1 += elapsed2;
        println!("zero_1 {} The time elapsed is: {} Ns.", zero_1, elapsed2);
        assert_eq!(zero as u32, zero_1);

    }
    println!(
        "1st func avg time elapsed is: {} Ns.\n2nd func avg time elapsed is: {} Ns.",
        avg_zero / 10000,
        avg_zero_1 / 10000,

    );
}


fn sol_1_utilizing_look_up_table(text: &str) -> u32 {

    fn take_exp_mod(exp: u32) -> u32 {
        let mut base: u32 = 2;
        let mut exp: u32 = exp;
        let mut t: u32 = 1;
        while exp > 0 {
            // for cases where exponent
            // is not an even value
            if exp % 2 != 0 {
                let temp:u64 =  (t as u64 * (base as u64)) % 1000000007;
                t = temp as u32;
            }
            let temp:u64 =  ((base as u64) * (base as u64)) % 1000000007;
            base = temp as u32;
            exp = exp / 2;
        }
        return t ;
    }

    let  mult = |a: u32, b: u32| -> u32 {
        let mut a:u64 = a as u64;
        let mut b:u64 = b as u64;
         
        ((a * b) % 1000000007) as u32
    };
    let mut look_up_table: [u32; 2] = [73741817, 147483634];// values of [ (2^30)% 1000000007   ,   (2^31)% 1000000007  ] 
    let mut latest_k = 31;
    let mut get_2_pow_k = |_k: u32| {
        if _k < 30 {
            return 1 << _k;
        } else {
            if _k == latest_k - 1 {
                return look_up_table[0];
            }

            if _k == latest_k {
                return look_up_table[1];
            }

            look_up_table[0] = look_up_table[1];
            look_up_table[1] = take_exp_mod(_k);
            latest_k += 1;
            return look_up_table[1]
        }
    };

    let mut nr_of_ones: u32 = 0;
    let mut res: u32 = 0;
    let mut k: u32 = 0;
    let mut secret_ingrediente = 0;

    let mut _2_pow_k = 1;

    for c in text.bytes() {
        if c == b'1' {
            nr_of_ones += 1;
        } else if c == b'0' {
            res =  (res + mult(_2_pow_k , nr_of_ones) + secret_ingrediente) % 1000000007;
        } else if c == b'?' {
            res =  ((res * 2) + mult(_2_pow_k , nr_of_ones) + secret_ingrediente) % 1000000007;
            k += 1;
            _2_pow_k = get_2_pow_k(k);
            secret_ingrediente = if k <= 0 {
                0
            } else {
                mult((k) , get_2_pow_k(k - 1))
            };
        }
    }
    res
}

fn sol2(text: &str) -> u64 {
    // THIS WILL NOT WORK FOR VALUES N> 2^32

    fn get_total_set_bits_0_to_n(m: u64) -> u64 {
        if m <= 0 {
            return 0;
        }
        // let b: u64 = log2(m)+1
        let b: u64 = (m).count_ones() as u64;

        // (1 << b) - 1   ==>   (2^b) -1
        // b * 2^(b-1)
        ((b) * (1 << (b - 1)) % 1000000007) % 1000000007
    }
    let mut nr_of_inversion: u64 = 0;
    let mut nr_of_ones: u64 = 0;
    let mut res: u64 = 0;
    let mut n: u64 = 1;
    let mut secret_ingrediente = 0;
    for c in text.bytes() {
        // because the value will be encoded in utf8, we have to take measures to ensure that the whole char is been read
        if c == b'1' {
            nr_of_ones += 1;
        } else if c == b'0' {
            // let mut secret_ingrediente:u64 = 0;
            // for i in 0..n {
            //     secret_ingrediente += (i as u32) .count_ones() as u64;
            // }
            res = res + (n * nr_of_ones as u64) + secret_ingrediente;
        } else if c == b'?' {
            // let mut secret_ingrediente:u64 = 0;
            // for i in 0..n {
            //     // (i as u32).count_ones() represent the number of ? which are 1's
            //     // if we have 3 "?" located before the curr char ,this will yeild the flwg possibilities: 000 001 010 011 100 101 110 111
            //     secret_ingrediente += (i as u32).count_ones() as u64;
            // }
            res = (res * 2) % 1000000007 + (n * nr_of_ones as u64) + secret_ingrediente;
            n *= 2;
            secret_ingrediente = get_total_set_bits_0_to_n(n - 1) as u64;
        }
    }

    nr_of_inversion = res;
    (nr_of_inversion % 1000000007) as u64
}


fn sol_alt(text: &str) -> i64 {
    let mut nr_of_inversion: u64 = 0;

    let mut nr_of_ones: u32 = 0;
    let mut res: Vec<u64> = vec![0];

    for c in text.bytes() {
        // because the value will be encoded in utf8, we have to take measures to ensure that the whole char is been read
        if c == b'1' {
            nr_of_ones += 1;
        } else if c == b'0' {
            for (i, val) in res.iter_mut().enumerate() {
                *val = *val + nr_of_ones as u64 + (i as u32).count_ones() as u64;
            }
        } else if c == b'?' {
            // let mut i: usize = 0;
            // let new_len = res.len() * 2;
            // let mut temp: Vec<u64> = vec![0; new_len as usize];
            // while i < res.len() {
            //     // (i as u32).count_ones() represent the number of ? which are 1's
            //     // if we have 3 "?" located before the curr char ,this will yeild the flwg possibilities: 000 001 010 011 100 101 110 111
            //     let curr: u64 = (res[i] + nr_of_ones as u64 + (i as u32).count_ones() as u64);
            //     let next: u64 = (res[i]);
            //     temp[i * 2] = curr;
            //     temp[(i * 2) + 1] = next;
            //     i += 1;
            // }
            let old_len = res.len();
            let new_len = old_len * 2;
            let mut temp: Vec<u64> = vec![0; new_len as usize];
            let mut i: usize = (old_len);
            while i > 0 {
                let prev: u64 = res.pop().unwrap();
                let curr = prev + nr_of_ones as u64 + ((i - 1) as u32).count_ones() as u64;
                let next = prev;
                temp[(i - 1) * 2] = curr;
                temp[((i - 1) * 2) + 1] = next;
                i -= 1;
            }
            res = temp;
        }
    }

    nr_of_inversion = res.iter().fold(0, |acc, x| acc + (*x as u64));
    (nr_of_inversion % 1000000007) as i64
}
