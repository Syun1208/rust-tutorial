use std::collections::HashMap;
use std::collections::HashSet;
/*
Collections:
    - Vector
        + new(): pub fn new()->Vect
        + push(): pub fn push(&mut self, value: T)
        + remove(): pub fn remove(&mut self, index: usize) -> T
        + contains(): pub fn contains(&self, x: &T) -> bool
        + len(): pub fn len(&self) -> usize
    - HashMap
        + insert(): pub fn insert(&mut self, k: K, v: V) -> Option
        + len(): pub fn len(&self) -> usize
        + get(): pub fn get<Q: ?Sized>(&lself, k: &Q) -> Option<&V> where K:Borrow Q:Hash+ Eq
        + iter(): pub fn iter(&self) -> Iter<K, V>
        + contains_key: pub fn contains_key<Q: ?Sized>(&self, k: &Q) -> bool
        + remove(): pub fn remove_entry<Q: ?Sized>(&mut self, k: &Q) -> Option<(K, V)>
    - HashSet
        + insert(): pub fn insert(&mut self, value: T) -> bool
        + len(): pub fn len(&self) -> usize
        + get(): pub fn get<Q:?Sized>(&self, value: &Q) -> Option<&T> where T: Borrow,Q: Hash + Eq
        + iter(): pub fn iter(&self) -> Iter
        + contains_key: pub fn contains<Q: ?Sized>(&self, value: &Q) -> bool
        + remove(): pub fn remove<Q: ?Sized>(&mut self, value: &Q) -> bool
*/

fn vector_example(){
    let mut v = Vec::new();
    v.push(20);
    v.push(30);
    v.push(40);
 
    println!("size of vector is :{}", v.len());
    println!("after pushing: {:?}", v);

    v.remove(1);
    println!("after removing index 1: {:?}",v);

    if v.contains(&20) {
        println!("found 20 in {:?}", v);
    }
}



fn hashmap_example(){
    let mut state_codes = HashMap::new();
    state_codes.insert("KL", "Kerala");
    state_codes.insert("MH", "Maharashtra");
    println!("{:?}", state_codes);

    println!("size of map is {}", state_codes.len());

    match state_codes.get(&"KL") {
        Some(value)=> {
           println!("Value for key KL is {}",value);
        }
        None => {
           println!("nothing found");
        }
    }

    for (key, val) in state_codes.iter() {
        println!("key: {} val: {}", key, val);
    }

    if state_codes.contains_key(&"KL") {
        println!("found key");
    }

    state_codes.remove(&"MH");
    println!("length of the hashmap after remove() {}", state_codes.len());
}



fn hashset_example(){
    let mut names = HashSet::new();

    names.insert("Mohtashim");
    names.insert("Kannan");
    names.insert("TutorialsPoint");
    names.insert("Mohtashim");//duplicates not added
 
    println!("{:?}",names);
    println!("size of the set is {}",names.len());
    for name in names.iter() {
        println!("{}",name);
    }

    match names.get(&"Mohtashim"){
        Some(value)=>{
           println!("found {}",value);
        }
        None =>{
           println!("not found");
        }
     }
    println!("{:?}",names);

    if names.contains(&"Kannan") {
        println!("found name");
    }  

    names.remove(&"Kannan");
    println!("length of the Hashset after remove() : {}",names.len());
}


fn main(){
    vector_example();
    hashmap_example();
    hashset_example();
}