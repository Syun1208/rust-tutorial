fn display_each_item_in_tuple(tuple: (i32,f64,u8)){
    println!("integer is : {:?}",tuple.0);
    println!("float is : {:?}",tuple.1);
    println!("unsigned integer is : {:?}",tuple.2);
}

fn display_full_tuple(tuple: (i32,f64,u8)){
    println!("Inside print method");
    println!("{:?}", tuple);
}

fn destruct_tuple(tuple: (i32,f64,u8)){
    println!("Inside print method");
    let (age, is_male, cgpa) = tuple; //assigns a tuple to distinct variables
    println!("Age is {} , isMale? {},cgpa is {}", age, is_male, cgpa);
}

fn main() {
    let tuple:(i32,f64,u8) = (-325,4.9,22);
    
    display_each_item_in_tuple(tuple);
    display_full_tuple(tuple);
    destruct_tuple(tuple);
 }