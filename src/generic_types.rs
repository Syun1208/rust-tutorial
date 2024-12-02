use std::fmt::Display;


struct Data<T> {
    value:T,
}

fn print_pro<T:Display>(t:T){
    println!("Inside print_pro generic function:");
    println!("{}", t);
}

fn main(){

    //generic type of i32
    let t:Data<i32> = Data{value: 350};
    println!("value is :{} ",t.value);

    //generic type of String
    let t2:Data<String> = Data{value:"Tom".to_string()};
    println!("value is :{} ",t2.value);

    print_pro(t.value);
    print_pro(t2.value);
 }