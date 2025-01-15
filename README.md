# guess-and-learn


## Índice de contenidos

* [Código inicial](#item_init)
* [Sobre enum](#item_enum)
* [Sobre usar o no ```expect```](#item_expect)
* [Librería std](#item_std)



<hr>

## **Juego de adivinanzas en Rust, para además aprender.**

<br>

<a id="item_init"></a>
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

Explicación del código (sólo las partes nuevas, no por ejemplo los ```println!```):

1. ```use std::io;``` esta parte es fácil, sólo hay que recordar que ```std``` es la biblioteca "standard" de Rust, e ```io``` es la biblioteca de entrada y salida de dentro de "standard" (io = input output). Esta biblioteca sirve para que el usuario imprima y/o lea datos.

2. ```let mut adivina = String::new();``` se crea una variable "string" vacía pero mutable (```String::new``` crea una instancia de ```String```).

3. ```io::stdin()``` llama a ```stdin``` de dentro del modulo "io".
    Stdin es un tipo que representa un "manejador" de la entrada estandar para un terminal.

4. Luego viene ```.read_line(&mut adivina)```, que es parte de ```stdin()```.
    ```.read_line(&mut adivina)``` llama al método read_line (un "manejador" de entrada del usuario) y pasamos cómo parametro la variable (referencia de ella pero mutable) antes creada, esto es para decirle que cadena se debe almacenar lo que el usuario escriba.

    Osea que, lo que hace esta línea es: darle una cadena mutable a un método, el cual añadirá lo que escriba el usuario a esta cadena.
    
    Tanto la parte del "read line" cómo la siguiente son hablando de lógica, la misma línea (todo podría ir en la misma línea de código Rust, pero por simplicidad se separa en tres).
    La línea de "read line" recibe un argumento, pero en este caso, también devuelve otro, y lo que devuelve es un [```enum```][enum-a] llamado ```Result``` (también puedes ver que es enum al final de este documento)

    ```Result``` tiene dos posibles variantes ```Ok``` y ```Err``` (error).
    Esto es parecido a JS cuando se llama a una API, la llamada puede ser exitosa (```Ok```) o puede ser fallida (```Err```).
    El error y tipo de error junto con toda su información se almacenan en la variable ```Err```, mientras que en ```Ok``` se almacena el valor generado existosamente.

5. El tipo ```Result``` al igual que otros tipos, tienen métodos predefinidos. 

    Uno de estos métodos se llama ```expect```, el cual sirve para que si ```Result``` devuelve ```Err``` entonces se ejecute el código dentro de ```expect```. (si no logras entender ```Result```, ve el final del documento)

    Osea que ```.expect("No se pudo leer la línea");```, sirve básicamente cómo un ```catch``` del ```try-catch```, el cual si el código dentro de ```try``` da un error ```catch``` devuelve un código o imprime en pantalla un error para ayudar al usuario.

Esta explicación finalizó.

Este código es parte de la función ```primer_adivinanza```.

Ahora vamos a implementar un número aleatorio en una nueva función para que la adivinanza sea realmente difícil, vamos a tener que adivinar un número del 1 al 100.

EN CONSTRUCCIÓN...


## Más sobre lo visto


<a id="item_enum"></a>

#### Enum

Enum es un tipo que puede estar en uno de varios estados posibles. Llamamos a cada estado posible una variante.

Ejemplo:
```
enum TipoDeIp {
    V4,
    V6,
}
```


<a id="item_expect"></a>

#### ```Expect```

Una cosa para saber sobre ```Expect``` es que si el programador no lo llama al usar ```Result``` entonces el programa seguirá ejecutandose, pero en la consola recibirá una advertencia donde se da a conocer que su uso es ampliamente recomendado.



<a id="item_std"></a>

#### Librería std

Rust entiende que puede ser muy lento y muy tedioso importar cada cosa que el progrador quiere traer desde la biblioteca "standard" de Rust.

Por ello desde Rust 2015 se trae algo muy bueno, un import que trae todo lo más usado en la mayoría de los programas.

Esto se llama prelude, es un conjunto de elementos predefinidos que se importan fácilmente.

Para saber más: [std::prelude][prelude].


#### Más info sobre ```Result```

```Result``` puede ser un poco difícil de entender, por ende te dejo otros lugares donde lograr entenderlo:

[Manejo de errores en Rust: una introducción a Result](https://blog.vasquezruiz.me/manejo-de-errores-en-rust-una-introduccion-a-result/)

## Bibliografía:


[enum-a]: https://book.rustlang-es.org/ch06-00-enums  "Enums y Pattern Matching"

[prelude]: https://doc.rust-lang.org/std/prelude/index.html  "Module prelude"