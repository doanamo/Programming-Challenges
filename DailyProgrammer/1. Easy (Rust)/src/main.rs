use std::io;
use std::io::Write;
use std::fs::File;

fn main()
{
    // Ask for name
    print!("What's your name? > ");
    let _ = io::stdout().flush();

    let mut name = String::new();
    io::stdin().read_line(&mut name).ok().expect("Failed to read line!");

    // Ask for age.
    print!("How old are you? > ");
    let _ = io::stdout().flush();

    let mut years = String::new();
    io::stdin().read_line(&mut years).ok().expect("Failed to read line!");

    // Ask for username.
    print!("What's your username? > ");
    let _ = io::stdout().flush();

    let mut username = String::new();
    io::stdin().read_line(&mut username).ok().expect("Failed to read line!");

    // Print formatted output.
    let output = format!("Your name is {}, you are {} years old, and your username is '{}'.", name.trim(), years.trim(), username.trim());

    println!("\n{}", output);

    // Write output to a file.
    let mut file = File::create("Output.txt").unwrap();
    file.write_all(output.as_bytes()).unwrap();
}
