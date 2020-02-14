use std::io ;

fn main() {
    let stdin = io::stdin();

    let mut text = String::new();
    stdin.read_line(&mut text).expect("Failed to read line");

    let mut sequences_list: Vec<Vec<char>> = vec![];

    gen_mutation(text, &mut sequences_list, 0);
    let mut total_nr_of_inversion = 0;
    for seq in sequences_list {
        total_nr_of_inversion += calc_nr_of_adjecent_swap(seq);
    }

        println!("{}", total_nr_of_inversion % 1000000007);

}

fn gen_mutation(text: String, sequences_list: &mut Vec<Vec<char>>, ind: usize) {
    if ind > text.len() {
        return;
    }

    let mut mutated: bool = false;

    for (i, c) in text.char_indices().take_while(|_| !mutated).skip(ind) {
        if c == '?' {
            mutated = true;
            let mutation1: String;
            let mutation2: String;
            if i < text.len() - 1 {
                mutation1 = format!("{}{}{}", &text[0..i], '0', &text[i + 1..]);
                mutation2 = format!("{}{}{}", &text[0..i], '1', &text[i + 1..]);
            } else {
                mutation1 = format!("{}{}", &text[0..i], '0');
                mutation2 = format!("{}{}", &text[0..i], '1');
            }

            gen_mutation(mutation1, sequences_list, i + 1);
            gen_mutation(mutation2, sequences_list, i + 1);
            break;
        }
    }

    if !mutated {
        sequences_list.push(text.chars().collect());
    }
}

fn calc_nr_of_adjecent_swap(seq: Vec<char>) -> i64 {
    let mut nr_of_inversion: i64 = 0;
    let mut fst_one: i64 = -1;

    for (i, c) in seq.iter().enumerate() {
        let i = i as i64;

        if *c == '1' && fst_one == -1 {
            fst_one = i;
        } else if *c == '0' && fst_one != -1 {
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
