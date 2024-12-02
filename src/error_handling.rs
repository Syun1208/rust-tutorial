use std::fs::File;

fn expect_example(){
    let f = File::open("pqr.txt").expect("File not able to open");
    //file does not exist
    println!("end of main");
}

fn unwrap_example(){
    fn is_even(no:i32)-> Result<bool,String> {
        if no%2==0 {
           return Ok(true);
        } else {
           return Err("NOT_AN_EVEN".to_string());
        }
    }
    let result = is_even(10).unwrap();
    println!("result is {}",result);
    println!("end of main");
}


fn main() {
   let f = File::open("main.jpg");   
   match f {
        Ok(f)=> {
            println!("file found {:?}", f);
        },
        Err(e)=> {
            println!("file not found \n{:?}", e);
        }
   }
   println!("end of main");

   expect_example();
   unwrap_example();
}