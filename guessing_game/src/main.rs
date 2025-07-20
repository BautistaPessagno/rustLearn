// uso el input/output library (io) la cual viene de la standar library
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("The guessing game");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess");

        //la funcion let sirve para definir variables
        //pero por defaulte no son mutable
        //si queres que sean mutable tenemos que usar mut
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess) //al igual que las variables, las referencias son inmutables, por eso
            //se le pasa con el &mut y no solo & para que sea mutable
            .expect("Failed to read line");

        //el String se pasa a u32
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        //usando {} lo que hacemos es pasar variables
        println!("your guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
