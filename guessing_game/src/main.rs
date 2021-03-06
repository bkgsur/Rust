extern crate rand;
use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1,101);
    //println!("The secret number is {}", secret_number);
    let mut attempts_count =0 ;
    loop
    {
        println!("Please input a guess");
        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Failed to read line");

        let guess: u32 = match guess.trim().parse()
        {
                Ok(num) => num,
                Err(msg) => 
                {
                    println!("Please input valid numbers! - {}", msg);
                    continue;
                }

        };
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number)
        {
            Ordering::Less => println!("Too Small"),
            Ordering::Greater => println!("Too Big"),
            Ordering::Equal => 
            {
                println!("You Win!");
                break;
            }

        }
        attempts_count+=1;
        println!("Attempt {}", attempts_count);
    }
}
