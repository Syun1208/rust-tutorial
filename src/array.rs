// pass by reference
fn update(arr: &mut [i32;3]){
    for i in 0..3 {
       arr[i] = 0;
    }

}

fn main() {
    let mut arr = [10,20,30];
    update(&mut arr);
    print!("Inside main {:?}", arr);
}