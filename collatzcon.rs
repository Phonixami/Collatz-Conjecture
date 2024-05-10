use std::{fs::File, io::{self, Write}};

fn main() {
    let mut input = String::new();
    print!("Please enter a positive integer: ");
    io::stdout().flush().expect("Could not flush stdout");
    io::stdin().read_line(&mut input).expect("Couldnt get input");
    let number = input.trim().parse::<u32>().expect("Input is not a positive integer");
    let mut file = File::create("collatz.txt").expect("Couldnt create file");
    for i in collatz(number) {
        let line = match i % 2 == 0 {
            true => format!("{} is Even \n", i),
            false => format!("{} is Odd \n", i),
        };
        file.write_all(line.as_bytes()).expect("Couldnt write to file");
    }

}

fn collatz(mut number: u32) -> Vec<u32> {
    let mut numbers: Vec<u32> = vec![];
    while number != 1 {
        numbers.push(number);
        match number % 2 == 0 {
            true => {
                println!("{} is Even", number);
                number /= 2;
            },
            false => {
                println!("{} is Odd", number);
                number = (number * 3) + 1;
            }
        }
    }
    numbers
}