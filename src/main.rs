use std::io;

fn main() {

    println!("=== CONVERSOR DE TEMPERATURA ===");

    loop {

        println!("\nElige la conversión:");
        println!("1. Celsius a Fahrenheit");
        println!("2. Fahrenheit a Celsius");
        println!("3. Salir");

        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        match choice.trim().parse() {
            Ok(1) => convertir_c_A_f(),
            Ok(2) => convertir_f_A_c(),
            Ok(3) => {
                println!("hasta luego");
                break;
            }
            _ => println!("por favor ingresa una opcion valida"),
        }

    }

}

fn convertir_c_A_f() {

    println!("Ingrese el valor en celsius");
    let celsius = obtener_numero();
    let fahrenheit = celsius * 1.8 + 32.0;
    println!("{}°C = {:.1}°F", celsius, fahrenheit);

}

fn convertir_f_A_c() {
    println!("Ingresa grados Fahrenheit:");
    let fahrenheit = obtener_numero();
    let celsius = (fahrenheit - 32.0) / 1.8;
    println!("{}°F = {:.1}°C", fahrenheit, celsius);
}

fn obtener_numero() -> f64 {

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input.trim().parse().expect("Please type a number!")
}
