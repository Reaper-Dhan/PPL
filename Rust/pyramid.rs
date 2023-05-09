use std::io;

fn main() {
    println!("Please enter the h of the pyramid:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let h: i32 = input.trim().parse().unwrap();
	println!();
    for i in 1..=h {
        for _j in 1..=h-i {
            print!(" ");
        }
        for _k in 1..=2*i-1 {
            print!("$");
        }
        println!();
    }
}

