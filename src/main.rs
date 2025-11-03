use std::io;

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
            Ok(1) => MenuTemp(),
            Ok(2) => MenuPeso(),
            Ok(3) => MenuLongitud(),
            Ok(4) => {
                println!("hasta luego");
                break;
            }
            _ => println!("por favor ingresa una opcion valida"),
        }
    }
}

fn MenuPeso() {

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
            Ok(1) => crate::convertir_kg_A_L(),
            Ok(2) => crate::convertir_L_A_Kg(),
            Ok(3) => crate::convertir_Kg_A_O(),
            Ok(4) => crate::convertir_O_A_Kg(),
            Ok(5) => main(),
            _ => println!("por favor ingresa una opcion valida"),
        }

    }
}

fn convertir_kg_A_L() {

    println!("Ingrese el valor en kilogramos");
    let kilogramos = obtener_numero();
    let libras = kilogramos * 2.20462;
    println!("{}Kg = {:.1}L", kilogramos, libras);
    MenuPeso();

}

fn convertir_L_A_Kg() {

    println!("Ingrese el valor en libras");
    let libras = obtener_numero();
    let kilogramos = libras / 2.20462;
    println!("{}L = {:.1}Kg", libras, kilogramos);
    MenuPeso();

}

fn convertir_Kg_A_O() {

    println!("Ingrese el valor en kilogramos");
    let kilogramos = obtener_numero();
    let onzas = kilogramos * 35.274;
    println!("{}Kg = {:.1}O", kilogramos, onzas);
    MenuPeso();

}

fn convertir_O_A_Kg() {

    println!("Ingrese el valor en onzas");
    let onzas = obtener_numero();
    let kilogramos = onzas / 35.274;
    println!("{}O = {:.1}Kg", onzas, kilogramos);
    MenuPeso();

}
fn MenuLongitud() {

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
            Ok(1) => crate::convertir_Mt_A_P(),
            Ok(2) => crate::convertir_P_A_Mt(),
            Ok(3) => crate::convertir_Km_A_Ml(),
            Ok(4) => crate::convertir_Ml_A_Km(),
            Ok(5) => main(),
            _ => println!("por favor ingresa una opcion valida"),
        }

    }
}

fn convertir_Mt_A_P() {

    println!("Ingrese el valor en Metros");
    let Metros = obtener_numero();
    let pies = Metros * 3.28084;
    println!("{}Mt = {:.1}P", Metros, pies);
    MenuLongitud();

}

fn convertir_P_A_Mt() {

    println!("Ingrese el valor en pies");
    let pies = obtener_numero();
    let Metros = pies / 3.28084;
    println!("{}P = {:.1}Mt", pies, Metros);
    MenuLongitud();

}

fn convertir_Km_A_Ml() {

    println!("Ingrese el valor en kilometros");
    let kilometros = obtener_numero();
    let millas = kilometros * 0.621371;
    println!("{}Km = {:.1}Ml", kilometros, millas);
    MenuLongitud();


}

fn convertir_Ml_A_Km() {

    println!("Ingrese el valor en millas");
    let millas = obtener_numero();
    let kilometros = millas / 0.621371;
    println!("{}Ml = {:.1}Km", millas, kilometros);
    MenuLongitud();


}
fn MenuTemp() {
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
            Ok(1) => convertir_c_A_f(),
            Ok(2) => convertir_f_A_c(),
            Ok(3) => convertir_c_A_k(),
            Ok(4) => convertir_k_A_c(),
            Ok(5) => main(),
            _ => println!("por favor ingresa una opcion valida"),
        }
    }
}
fn convertir_c_A_f() {
    println!("Ingrese el valor en celsius");
    let celsius = obtener_numero();
    let fahrenheit = celsius * 1.8 + 32.0;
    println!("{}°C = {:.1}°F", celsius, fahrenheit);
    MenuTemp();
}

fn convertir_f_A_c() {
    println!("Ingresa grados Fahrenheit:");
    let fahrenheit = obtener_numero();
    let celsius = (fahrenheit - 32.0) / 1.8;
    println!("{}°F = {:.1}°C", fahrenheit, celsius);
    MenuTemp();
}

fn convertir_c_A_k() {

    println!("Ingresa grados celsius:");
    let celsius = obtener_numero();
    let kelvin = celsius + 273.5;
    println!("{}°C = {:.1}°K", celsius, kelvin);
    MenuTemp();

}

fn convertir_k_A_c() {

    println!("Ingresa grados kelvin:");
    let kelvin = obtener_numero();
    let celsius = kelvin - 273.5;
    println!("{}°K = {:.1}°C", kelvin, celsius);
    MenuTemp();

}

fn obtener_numero() -> f64 {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input.trim().parse().expect("Please type a number!")
}