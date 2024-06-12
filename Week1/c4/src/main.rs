fn main() {
    let arr = [10, 11, 12, 9, 10];
    println!("{:?}", progress_days(&arr));
}

fn progress_days(arr: &[i32]) -> i32{
    let mut progress = 0;
    for i in 0..arr.len()-1{
        if arr[i] < arr[i+1]{
            progress += 1;
        }
    }
    return progress;
}