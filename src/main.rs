mod temperatura;
mod peso;
mod longitud;

use std::io;
use temperatura::{convertir_c_a_f, convertir_c_a_k, convertir_f_a_c, convertir_k_a_c};
use longitud::{convertir_km_a_ml, convertir_ml_a_km, convertir_p_a_mt, convertir_mt_a_p};
use peso::{convertir_kg_a_l, convertir_l_a_kg, convertir_o_a_kg, convertir_kg_a_o};

fn main() {

    println!("=== CONVERSOR DE TEMPERATURA Y PESO ===");

    loop {

        println!("\nElige la opcion:");
        println!("1. conversor de temperatura");
        println!("2. conversor de peso");
        println!("3. conversor de longitud");
        println!("4. Salir");

        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        match choice.trim().parse() {
            Ok(1) => {
                menu_temp();
                break;
            }
            Ok(2) => {
                menu_peso();
                break;
            }
            Ok(3) => {
                menu_longitud();
                break;
            }
            Ok(4) => {
                println!("hasta luego");
                break;
            }
            _ => println!("por favor ingresa una opcion valida"),
        }
    }
}

fn menu_peso() {

    println!("=== MENU DE PESO ===");

    loop {

        println!("\nElige la conversión:");
        println!("1. Kilogramos a Libras");
        println!("2. Libras a Kilogramos");
        println!("3. Kilogramos a Onzas");
        println!("4. Onzas a Kilogramos");
        println!("5. volver");

        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        match choice.trim().parse() {
            Ok(1) => {
                convertir_kg_a_l();
                break;
            }
            Ok(2) => {
                convertir_l_a_kg();
                break;
            }
            Ok(3) => {
                convertir_kg_a_o();
                break;
            }
            Ok(4) => {
                convertir_o_a_kg();
                break;
            }
            Ok(5) => {
                main();
                break;
            }
            _ => println!("por favor ingresa una opcion valida"),
        }

    }
}


fn menu_longitud() {

    println!("=== MENU DE LONGITUD ===");

    loop {

        println!("\nElige la conversión:");
        println!("1. Metros a Pies");
        println!("2. Pies a Metros");
        println!("3. Kilometros a Millas");
        println!("4. Millas a Kilometros");
        println!("5. volver");

        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        match choice.trim().parse() {
            Ok(1) => {
            convertir_mt_a_p();
            break;
        }
            Ok(2) => {
                convertir_p_a_mt();
                break;
            }
            Ok(3) => {
                convertir_km_a_ml();
                break;
            }
            Ok(4) => {
                convertir_ml_a_km();
                break;
            }
            Ok(5) => {
                main();
                break;
            }
            _ => println!("por favor ingresa una opcion valida"),
        }

    }
}


fn menu_temp() {
    println!("=== MENU DE TEMPERATURA ===");

    loop {
        println!("\nElige la conversión:");
        println!("1. Celsius a Fahrenheit");
        println!("2. Fahrenheit a Celsius");
        println!("3. Celsius a Kelvin");
        println!("4. Kelvin a Celsius");
        println!("5. volver");

        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        match choice.trim().parse() {
            Ok(1) => {
                convertir_c_a_f();
                break;
            }
            Ok(2) => {
                convertir_f_a_c();
                break;
            }
            Ok(3) => {
                convertir_c_a_k();
                break;
            }
            Ok(4) => {
                convertir_k_a_c();
                break;
            }
            Ok(5) => {
                main();
                break;
            }
            _ => println!("por favor ingresa una opcion valida"),
        }
    }
}


fn obtener_numero() -> f64 {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input.trim().parse().expect("Please type a number!")
}