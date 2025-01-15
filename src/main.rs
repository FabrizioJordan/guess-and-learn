use std::io;

fn main() {
    println!("Adivina el número!");

    println!("Por favor, escríbe el número: ");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("No se pudo leer la línea");

    println!("Advivinaste!! El número era : {}", guess);
}