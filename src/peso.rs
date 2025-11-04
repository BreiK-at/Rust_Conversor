use crate::{obtener_numero, menu_peso};

pub fn convertir_kg_a_l() {

    println!("Ingrese el valor en kilogramos");
    let kilogramos = obtener_numero();
    let libras = kilogramos * 2.20462;
    println!("{}Kg = {:.1}L", kilogramos, libras);
    menu_peso();

}

pub fn convertir_l_a_kg() {

    println!("Ingrese el valor en libras");
    let libras = obtener_numero();
    let kilogramos = libras / 2.20462;
    println!("{}L = {:.1}Kg", libras, kilogramos);
    menu_peso();

}

pub fn convertir_kg_a_o() {

    println!("Ingrese el valor en kilogramos");
    let kilogramos = obtener_numero();
    let onzas = kilogramos * 35.274;
    println!("{}Kg = {:.1}O", kilogramos, onzas);
    menu_peso();

}

pub fn convertir_o_a_kg() {

    println!("Ingrese el valor en onzas");
    let onzas = obtener_numero();
    let kilogramos = onzas / 35.274;
    println!("{}O = {:.1}Kg", onzas, kilogramos);
    menu_peso();

}