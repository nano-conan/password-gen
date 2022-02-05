use std::io;
use std::io::Write;

fn main() {
    println!("Password Generator!");
    print!("How many characters does your password need to be? ");

    let mut input = String::new();
    let _ = io::stdout().flush();

    io::stdin()
        .read_line(&mut input)
        .expect("error: unable to read user input");

    let char_length: i32 = input.trim().parse::<i32>().unwrap();

    let mut count = 0;

    let mut pw = String::new();

    loop{
        count += 1;

        pw = format!("{}{}",pw, rand::random::<char>());

        if count >= char_length
        {
            break;
        }
    }

    println!("Your pw is : {}", pw);
}

