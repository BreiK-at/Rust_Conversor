use crate::{obtener_numero, menu_temp};

pub fn convertir_c_a_f() {
    println!("Ingrese el valor en celsius");
    let celsius = obtener_numero();
    let fahrenheit = celsius * 1.8 + 32.0;
    println!("{}°C = {:.1}°F", celsius, fahrenheit);
    menu_temp();
}

pub fn convertir_f_a_c() {
    println!("Ingresa grados Fahrenheit:");
    let fahrenheit = obtener_numero();
    let celsius = (fahrenheit - 32.0) / 1.8;
    println!("{}°F = {:.1}°C", fahrenheit, celsius);
    menu_temp();
}

pub fn convertir_c_a_k() {

    println!("Ingresa grados celsius:");
    let celsius = obtener_numero();
    let kelvin = celsius + 273.5;
    println!("{}°C = {:.1}°K", celsius, kelvin);
    menu_temp();

}

pub fn convertir_k_a_c() {

    println!("Ingresa grados kelvin:");
    let kelvin = obtener_numero();
    let celsius = kelvin - 273.5;
    println!("{}°K = {:.1}°C", kelvin, celsius);
    menu_temp();

}