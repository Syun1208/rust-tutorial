use std::any::type_name;
use std::fmt::Display;

fn print_type<T>(_: &T) {
    println!("Type: {}", type_name::<T>());
}

fn show_tokens<T: Display>(tokens: &Vec<T>) {
    let mut i = 1;
    for token in tokens {
        println!("token {}: {}", i, token);
        i += 1;
    }
}


fn main() {
    // String Literal
    /* 
        static owner:&str = "Hello world";
        let owner:&'static str = "Hello world";
    */
    let owner:&str = "Hello World";
    println!("Owner -> String Literal {}", owner);

    // String Object
    /*
        Initialize the object;
            + String::new();
            + String::from();
        Methods:
            + new()
            + to_string(): convert string literal to string object
            + replace()
            + as_str(): the inverse of to_string()
            + push()
            + push_str()
            + len()
            + trim()
            + split_whitespace()
            + split()
            + chars()
    */
    let mut empty_string:String = String::new();
    let content:String = String::from(
        "\n\tMr.Long currently works as a ML Engineer \t Software Engineer \n
        In the future, he wanna become an AI Solution Architect and Full Stack Data Scientist"
    );

    // Add string
    empty_string.push_str(" Mr.Long, from Nexcel Solution");

    // Add char
    empty_string.push('s');

    //Replace
    let replace_string:String = empty_string.replace("Long", "Leon");
    
    // Get string length
    println!("{}", replace_string.len());

    // Remove the whitespace
    let trim_string = empty_string.trim();
    println!("{}", trim_string.len());

    // Convert string object to string literal
    let literal:&str = content.as_str();
    print_type(&literal);

    // Convert string literal to string object
    let object:String = literal.to_string();
    print_type(&object);

    // Splited by whitespace
    let content_split_whitespace = content.split_whitespace();
    let content_as_list: Vec<&str> = content_split_whitespace.clone().collect();
    println!("Tokens: {:?}", content_as_list);

    show_tokens(&content_as_list);

    // Splited by character or string
    let replace_string_split = replace_string.split(",");
    let replace_string_as_list: Vec<&str> = replace_string_split.clone().collect();
    println!("Tokens: {:?}", replace_string_as_list);

    show_tokens(&replace_string_as_list);
    
    // Get all characters in string
    let content_chars = content.chars();
    let content_chars_as_list: Vec<char> = content_chars.clone().collect();
    println!("Tokens: {:?}", content_chars_as_list);

    show_tokens(&content_chars_as_list)
}