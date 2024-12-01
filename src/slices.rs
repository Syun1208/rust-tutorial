fn use_slice(slice:&mut [i32]) {
    println!("length of slice is {:?}",slice.len());
    println!("{:?}",slice);
    slice[0] = 1010; // replaces 20 with 1010
}


fn main(){
    let mut data = [10,20,30,40,50];
    use_slice(&mut data[1..4]); // passes references of 20, 30 and 40
    println!("{:?}",data);
}