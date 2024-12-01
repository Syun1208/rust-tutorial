fn for_loop(){
    for x in 1..11{ // 11 is not inclusive
        if x==5 {
           continue;
        }
        println!("x is {}",x);
     }
}

fn while_loop(){
    let mut x = 0;
    while x < 10{
       x+=1;
       println!("inside loop x value is {}",x);
    }
    println!("outside loop x value is {}",x);
}

fn main(){
    for_loop();
    while_loop();
 }