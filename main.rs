use std::io;

fn main() {
    println!("Enter lower bound (inclusive)");
    let lower_bound = parse_input(String::new());
    println!("Enter upper bound (inclusive)");
    let upper_bound = parse_input(String::new());
    println!("Enter multiple");
    let multiple = parse_input(String::new());

    let dash = "-";
    println!("{}", dash.repeat(50));

    let mut total = 0;
    
    for i in lower_bound..=upper_bound {
        if multiple != 0 {
            if i % multiple == 0 && i != 0 {
                println!("{}", i);
                total += 1;
            }
        }
    }

    println!("{}", dash.repeat(50));
    println!("{} multiples of {} in the range of {} - {}", total, multiple, lower_bound, upper_bound);
}

fn parse_input(mut input: String) -> i32 {
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    
    let input = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => bad_input(),
    };

    input
}

fn bad_input() -> i32 {
    println!("Not an integer. Defaulting to 0");
    0
}
