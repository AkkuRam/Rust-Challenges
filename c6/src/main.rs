fn main() {
    next_prime(26);
}

fn is_prime(n: i32)->bool{
    for i in 2..n/2{
        if n % i == 0{
            return false;
        }
    }
    return true;
}

fn next_prime(mut n: i32){
    if is_prime(n){
        print!("The number {} is prime", n);
    }else{
        while !is_prime(n){
            n += 1;
        }
        print!("The number {} is prime", n);
    }
}
