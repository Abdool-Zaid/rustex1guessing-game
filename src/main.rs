use std::io;
use std::cmp::Ordering;
use rand::Rng;
fn main() {
    
    // init game
    println!("guess a number between 1 and 10 \nplease input your guess");
   
    let ans=rand::thread_rng().gen_range(1..=10);

    loop{
        let mut choice = String::new();
    // let res =natural();


    // bind choice to user variable
         io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");


            // convert type
            let choice: u32 = choice.trim().parse().expect("Please type a number!");
            
            println!("you guessed {choice} ");
            
            match choice.cmp(&ans) {
                Ordering::Less => println!("Too small!\n"),
                Ordering::Greater => println!("Too big!\n"),
                Ordering::Equal => {
                    println!("You win!\n");
                    break;
                }
            }
        }






}
