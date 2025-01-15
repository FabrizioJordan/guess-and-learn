# guess-and-learn

Juego de adivinanzas en Rust, para además aprender.

Código inicial:

```
use std::io;

fn main() {
    println!("Adivina el número!");

    println!("Por favor, escríbe el número: ");

    let mut adivina = String::new();

    io::stdin()
        .read_line(&mut adivina)
        .expect("No se pudo leer la línea");

    println!("Advivinaste!! El número era : {}", adivina);
}
```

Explicación del código:

1. ```use std::io;``` esta parte es fácil, sólo hay que recordar que ```std``` es la biblioteca "standard" de Rust, e ```io``` es la biblioteca de entrada y salida de dentro de "standard" (io = input output). Esta biblioteca sirve para que el usuario imprima y/o lea datos.

2. ```let mut adivina = String::new();``` se crea una variable "string" vacía pero mutable (```String::new``` crea una instancia de ```String```).

3. ```io::stdin()``` llama a ```stdin``` de dentro del modulo "io".
    Stdin es un tipo que representa un "manejador" de la entrada estandar para un terminal.

4. Luego viene ```.read_line(&mut adivina)```, que es parte de ```stdin()```.
    ```.read_line(&mut adivina)``` llama al método read_line (un "manejador" de entrada del usuario) y pasamos cómo parametro la variable (referencia de ella pero mutable) antes creada, esto es para decirle que cadena se debe almacenar lo que el usuario escriba.
    Osea que, lo que hace esta línea es: darle una cadena mutable a un método, el cual añadirá lo que escriba el usuario a esta cadena.

5. Tanto la parte del "read line" cómo la siguiente son hablando de lógica, la misma línea (todo podría ir en la misma línea de código Rust, pero por simplicidad se separa en tres).
    La línea de "read line" recibe un argumento, pero en este caso, también devuelve otro, y lo que devuelve es un [```enum```](https://book.rustlang-es.org/ch06-00-enums) llamado ```Result```
    



## Últimas cosas

Rust entiende que puede ser muy lento y muy tedioso importar cada cosa que el progrador quiere traer desde la biblioteca "standard" de Rust.

Por ello desde Rust 2015 se trae algo muy bueno, un import que trae todo lo más usado en la mayoría de los programas.

Esto se llama prelude, es un conjunto de elementos predefinidos que se importan fácilmente.

Para saber más: [std::prelude](https://doc.rust-lang.org/std/prelude/index.html).