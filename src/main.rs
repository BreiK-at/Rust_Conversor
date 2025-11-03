use std::io;

fn main() {

    println!("=== CONVERSOR DE TEMPERATURA ===");

    println!("Ingresa el valor en grados Celcius");

    let mut input = String::new();

    io::stdin()

        .read_line(&mut input)
        .expect("Failed to read line");

    let celsius: f64 =

        input.trim()
        .parse()
        .expect("Please type a number!");

    let fahrenheit = celsius * 1.8 + 32.0;

    println!("{}°C = {}°F", celsius, fahrenheit);

}
