use std::io;

fn main() {
    let stdin = io::stdin();

    // let mut text = String::new();
    // stdin.read_line(&mut text).expect("Failed to read line");
    let text = "11?10?0".to_string();

    //                       let placeholder_indicies:Vec<u32> = find_placeholder_indicies(&text);
    //                       let nr_of_placeholder = placeholder_indicies.len();
    let nr_of_placeholder:usize = find_nr_of_placeholders(&text);


    let combi = (2 as usize).pow(nr_of_placeholder as u32);

    
    let mut total_nr_of_inversion = 0;

    for ind in 0..combi {
        let placeholder_seq: Vec<char> = generate_placeholder_seq(nr_of_placeholder, ind).chars().collect();
        // println!("placeholder_seq {:?}", placeholder_seq );

        let mut a_seq = String::new();
        let mut placeholder_nr = 0;
        for (i, c) in text.char_indices() {
            if c == '?' {
                a_seq.push(placeholder_seq[placeholder_nr]);
                placeholder_nr += 1;
            } else {
                a_seq.push(c);
            }
        }
        // println!("{}", a_seq);
        let temp_nr_of_inversion = calc_nr_of_adjecent_swap(&a_seq);
        // println!("{}", temp_nr_of_inversion);
        total_nr_of_inversion += temp_nr_of_inversion;
    }
    println!("{}", total_nr_of_inversion % 1000000007);

}

fn find_nr_of_placeholders(text: &str) -> usize {
    let mut nr_of_placeholder = 0;
    for c in text.chars() {
        if c == '?' {
            nr_of_placeholder += 1;
        }
    }
    nr_of_placeholder
}


fn generate_placeholder_seq(nr_of_bits: usize, order: usize) -> String {
    // for i in 0..nr_of_bits {
    // let b = format!("{:01$b}", order, nr_of_bits);
    // println!("{}", b);
    // }
    format!("{:01$b}", order, nr_of_bits)
}



fn calc_nr_of_adjecent_swap(seq: &str) -> i64 {
    let mut nr_of_inversion: i64 = 0;
    let mut fst_one: i64 = -1;

    for (i, c) in seq.char_indices() {
        let i = i as i64;

        if c == '1' && fst_one == -1 {
            fst_one = i;
        } else if c == '0' && fst_one != -1 {
            nr_of_inversion = nr_of_inversion + i - fst_one;
            fst_one += 1;
        }
    }

    // println!("{:?}", seq);
    // println!("---------------------------");
    // println!("{}", nr_of_inversion);
    // println!("---------------------------");
    nr_of_inversion
}




