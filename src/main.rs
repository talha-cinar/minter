use std::env;
use std::fs;

fn main() 
{
    let args: Vec<String> = env::args().collect();
    if args.len()!=2
    {
        println!("Provide a file");
        return;
    }

    let input_file = &args[1];

    let input = fs::read_to_string(input_file)
        .expect("\nLine 16 .expect\n");

    println!("{}",input);

    let mut output = String::new();

    for i in input.chars()
    {
        if !i.is_whitespace()
        {
            print!("{}",i);
            output.push(i);
        }
    }    
    let output_file = "/home/user/minsomefile";

    fs::write(output_file, output)
        .expect("Error on line 33 .expect");
}


