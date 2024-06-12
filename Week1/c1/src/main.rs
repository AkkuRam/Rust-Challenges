fn main() {
    print!("Age in days: {}\n", calc_age(20));
}

fn calc_age(age: i32) -> i32{
    return age * 365
}
