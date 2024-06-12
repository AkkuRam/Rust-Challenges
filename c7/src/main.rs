fn main() {
    let arr1 = [1,2,3,0,0,0];
    let arr2 = [2,5,6];
    let m = 3;
    let n = 3;
    sort_array(arr1.to_vec(), arr2.to_vec(), m, n);
}

fn sort_array(mut arr1: Vec<i32>, arr2: Vec<i32>, m: usize, n: usize)->bool{
    if arr1.len() != m+n{
        return false;
    }

    if arr2.len() != n{
        return false;
    }

    arr1.retain(|&num| num != 0);
    let mut new_arr = arr1.clone( );
    new_arr.extend(arr2);
    new_arr.sort();
    
    println!("{:?}", new_arr);
    return true;
}