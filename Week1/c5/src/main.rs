use std::collections::HashMap;

fn main() {
    let string = "CABBACCC";
    sock_pairs(&string);
}

fn sock_pairs(s: &str){
    let mut char_counts: HashMap<char, usize> = HashMap::new();
    let mut count = 0;
    for c in s.chars() {
        *char_counts.entry(c).or_insert(0) += 1;
    }

    for val in char_counts.values(){
        if val % 2 == 0{
            count += val/2;
        }
    }
    
    print!("{:?}", count)
}