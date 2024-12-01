const PI: f32 = 3.14;

// define a function
fn fn_hello(){
    println!("Be tap code rust ^.^");
 }

// return a value in function
fn compute_volumne_cicle(r: f32) -> f32{
    return (4.0 / 3.0) * PI * r.powf(3.0);
}

// pass by value
fn pass_by_value_radius(mut r: f32){
    r = 32.0;
    println!("Value r to {}", r);
}

// pass by reference
fn pass_by_ref_radius(r: &mut f32){
    *r = 32.0;
    println!("Value r to {}", r);
}

fn display(param_name:String){
    println!("param_name value is : {}",param_name);
 }


fn main(){
    fn_hello();
    let mut r: f32 = 3.5;
    let v: f32 = compute_volumne_cicle(r);
    println!("{}", v);

    pass_by_value_radius(r);
    let v: f32 = compute_volumne_cicle(r);
    println!("{}", v);

    pass_by_ref_radius(&mut r);
    let v: f32 = compute_volumne_cicle(r);
    println!("Update volume to {}", v);

    let name:String = String::from("Leon");
    display(name);
}