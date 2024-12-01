fn main() {
    // Immutable: the variable's value cannot be changed 
    let _owner:&str = "Anh Long von di da dep trai sieu cap vu tru";
    // owner = "Anh Long xau trai vai noi"; // error[E0384]: re-assignment of immutable variable `owner`

    // Mutable: the value of a mutable variable can be changed.
    let mut owner:&str = "Anh Long von di da dep trai sieu cap vu tru";
    owner = "Anh thich mot co gai co ten viet tat la CN ^.^";
    println!("{}", owner);
}