use std::io;

fn fizzbuzz(n: u32) {
    for i in 1..n+1{
        match i {
            i if (i%15==0) => println!("FizzBuzz"),
           i if (i%3==0) => println!("Fizz"),
           i if (i%5==0) => println!("Buzz"),          
            _=> println!("{}",i),
        }
    }
}

fn main() {
    println!("Hello, FizzBuzz!");
    println!("Please type a positive integer:");

    let mut user_input = String::new();
    
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");
    
    let user_input: u32 = user_input.trim().parse().expect("Please type a integer!");

    println!("Your input is: {user_input}");
    
    println!("The FizzBuzz result for {user_input} is: ");
    fizzbuzz(user_input);   
        
}
       
