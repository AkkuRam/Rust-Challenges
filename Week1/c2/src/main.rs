fn main() {
    find_nemo("Where is Nemo lol")
}

fn find_nemo(string: &str){
   let substring = "Nemo";
   let arr = string.split_whitespace().collect::<Vec<_>>();
   let pos = arr.iter().position(|&word| word == substring);
   print!("I found Nemo in the position {:?}", pos.unwrap() + 1)
}

