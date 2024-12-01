fn is_prime(n: i32) -> bool {
    if n <= 1 {
        return false;
    }
    for i in 2..=(n as f64).sqrt() as i32 {
        if n % i == 0 {
            return false;
        }
    }
    return true
}

fn prime_checker(numbers: &Vec<i32>) -> Vec<i32> {
    let mut list_prime: Vec<i32> = Vec::new();
    for &number in numbers {
        // Decision Making
        if is_prime(number) {
            list_prime.push(number);
        }
    }
    list_prime
}

fn main() {
    let numbers: Vec<i32> = (1..=100).collect(); // Generate numbers from 1 to 100
    let list_prime = prime_checker(&numbers);   // Get the list of prime numbers

    println!("List of prime numbers: {:?}", list_prime);
}
