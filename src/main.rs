use std::io;

fn main() {
    println!("\n");
    println!("Bienvenido a este juego de adivinanza");
    println!("\n");

    //primer_adivinanza();
    //segunda_adivinanza();
    //tercer_adivinanza();
    ultima_adivinanza();
}
/* 
fn primer_adivinanza(){
    println!("Adivina el número!");

    println!("Por favor, escríbe el número: ");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("No se pudo leer la línea");

    println!("Advivinaste!! El número era : {}", guess);
}
*/

use rand::Rng;

/* 
fn segunda_adivinanza() {
    println!("Adivina el número!");

    let azar_numero = rand::thread_rng().gen_range(1..=100);

    println!("El número al azar es: {azar_numero}");

    println!("Por favor, escríbe el número: ");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("No se pudo leer la línea");

    println!("El número era: {guess}");
}
*/

use std::cmp::Ordering;

/*
fn tercer_adivinanza(){
    // este código no se ejecutará, está incompleto

    println!("Elegiste el número: {guess}");

    match guess.cmp(&num_secreto) {
        Ordering::Greater => println!("Muy grande!"),
        Ordering::Less => println!("Muy pequeño!"),
        Ordering::Equal => println!("Ganaste! El número era: {} ", &num_secreto),
    }
}
*/

/* 
fn tercer_adivinanza(){
    let num_secreto = rand::thread_rng().gen_range(1..=100);

    println!("Por favor, escríbe el número: ");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("No se pudo leer la línea");

    let guess: u32 = guess.trim().parse().expect("Por favor, escríbe un número");
    
    println!("Elegiste el número: {guess}");

    println!("\n");
    
    match guess.cmp(&num_secreto) {
        Ordering::Greater => println!("Muy grande!"),
        Ordering::Less => println!("Muy pequeño!"),
        Ordering::Equal => println!("Ganaste!"),
    }
    
    println!("El número era: {} ", &num_secreto)

}
*/

/* 
fn ultima_adivinanza(){

    loop {
        let num_secreto = rand::thread_rng().gen_range(1..=100);

        println!("Por favor, escríbe el número: ");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("No se pudo leer la línea");

        let guess: u32 = guess.trim().parse().expect("Por favor, escríbe un número");
        
        println!("Elegiste el número: {guess}");

        println!("\n");
        
        match guess.cmp(&num_secreto) {
            Ordering::Greater => println!("Muy grande!"),
            Ordering::Less => println!("Muy pequeño!"),
            Ordering::Equal => println!("Ganaste!"),
        }

        println!("El número era: {} ", &num_secreto);

    }
   
}
*/

fn ultima_adivinanza(){

    loop {
        let num_secreto = rand::thread_rng().gen_range(1..=100);

        println!("Por favor, escríbe el número: ");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("No se pudo leer la línea");

        let guess: u32 = guess.trim().parse().expect("Por favor, escríbe un número");
        
        println!("Elegiste el número: {guess}");

        println!("\n");
        
        match guess.cmp(&num_secreto) {
            Ordering::Greater => println!("Muy grande!"),
            Ordering::Less => println!("Muy pequeño!"),
            Ordering::Equal => {
                println!("Ganaste!");
                println!("El número era: {} ", &num_secreto);
                break;
            }
        }

        println!("El número era: {} ", &num_secreto);

    }
   
}