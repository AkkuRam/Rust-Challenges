fn main() {
    let arr =  [
        "--oooo-ooo--",
        "--xxxxxxxx--",
        "--o---",
        "-o-----o---x--",
        "--o---o-----"
     ] ;
    barbecue_skewers(&arr);
}

fn barbecue_skewers(arr: &[&str]){
    let mut veg = 0;
    let mut nonveg = 0;
    let mut vec = Vec::new();
    for part in arr{
        if part.contains("x"){
            nonveg += 1;
        }else{
            veg += 1;
        }
    }
    vec.push(veg);
    vec.push(nonveg);
    print!("{:?}", vec);
}