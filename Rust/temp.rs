use std::io;

fn c_to_f(c: f64) -> f64 {
    c * 9.0 / 5.0 + 32.0
}

fn f_to_c(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

fn main() {
    println!("Choose conversion:");
    println!("1 => Celsius to Fahrenheit");
    println!("2 => Fahrenheit to Celsius");

    let mut choice_input = String::new();
    io::stdin().read_line(&mut choice_input).unwrap();
    let choice: i32 = choice_input.trim().parse().unwrap();

    println!("Enter a temperature: ");
    let mut temp_input = String::new();
    io::stdin().read_line(&mut temp_input).unwrap();
    let temp: f64 = temp_input.trim().parse().unwrap();

    let f = c_to_f(temp);
    let c = f_to_c(temp);

    if choice == 1 {
        println!("{:.2} degrees C => {:.2} degrees F", temp, f);
    } else if choice == 2 {
        println!("{:.2} degrees F => {:.2} degrees C", temp, c);
    } else {
        println!("Invalid choice!");
    }
}

