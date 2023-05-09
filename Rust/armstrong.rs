use std::io;

fn main() {
    println!("Enter a number:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let number: u32 = input.trim().parse().unwrap();
    let mut sum = 0;
    let mut temp = number;

    while temp != 0 {
        let digit = temp % 10;
        sum += digit.pow(3);
        temp /= 10;
    }

    if number == sum {
        println!("{} is an Armstrong number", number);
    } else {
        println!("{} is not an Armstrong number", number);
    }
}

