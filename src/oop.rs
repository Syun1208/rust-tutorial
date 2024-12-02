//declare a structure
struct Book {
    name:&'static str,
    id:u32
}



//declare a interface
trait Printable {
    fn print(&self);
}



//implement the interface
impl Printable for Book {
    fn print(&self){
        println!("Printing book with id:{} and name {}",self.id,self.name)
    }
}



fn main(){
    //create an instance of the structure
    let b1 = Book {
       id:1001,
       name:"Rust in Action"
    };

    b1.print();
}
