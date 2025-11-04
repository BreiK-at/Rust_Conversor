use crate::{obtener_numero, menu_longitud};

pub fn convertir_mt_a_p() {

    println!("Ingrese el valor en Metros");
    let metros = obtener_numero();
    let pies = metros * 3.28084;
    println!("{}Mt = {:.1}P", metros, pies);
    menu_longitud();

}

pub fn convertir_p_a_mt() {

    println!("Ingrese el valor en pies");
    let pies = obtener_numero();
    let metros = pies / 3.28084;
    println!("{}P = {:.1}Mt", pies, metros);
    menu_longitud();

}

pub fn convertir_km_a_ml() {

    println!("Ingrese el valor en kilometros");
    let kilometros = obtener_numero();
    let millas = kilometros * 0.621371;
    println!("{}Km = {:.1}Ml", kilometros, millas);
    menu_longitud();


}

pub fn convertir_ml_a_km() {

    println!("Ingrese el valor en millas");
    let millas = obtener_numero();
    let kilometros = millas / 0.621371;
    println!("{}Ml = {:.1}Km", millas, kilometros);
    menu_longitud();


}