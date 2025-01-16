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


## Segundo juego, con números random

Este código va a ser parte de la función ```primer_adivinanza```.

Ahora vamos a implementar un número aleatorio en una nueva función para que la adivinanza sea realmente difícil, vamos a tener que adivinar un número del 1 al 100.

Rust realmente no tiene un método base para aleatoriedad, pero esto se resuelve con una crate.

Una crate es algo que siempre usamos en Rust todos nosotros, es algo así como el archivo o los archivos que van con tu código [¹](#bib_crate1) , o dicho de otra forma, es un paquete que puede contener el código fuente [²](#bib_crate2).    

Específicamente lo que estamos construyendo es un crate binario, osea un ejecutable, mientras que el crate que se suele usar para crear aleatoriedad se llama ```rand```, el cual es un crate de tipo librería, osea que no se puede ejecutar directamente, sino que se usa en otros programas.

Si conoces JavaScript y cómo ```packages.json``` es el archivo para organizar las dependencias de nuestro proyecto, entonces te servirá para entender cómo en Rust se usa el archivo ```Cargo.toml```.

Ahora hay que añadir el crate ```rand``` cómo dependencia dentro de nuestro archivo de dependencias.

Nuestro archivo ahora va a contener no solo información sobre nuestro paquete sino también sobre nuestras dependencias.

Así se tiene que ver la zona de dependencias:

```
[dependencies]
rand = "0.8.5"
```

Esto es básicamente:

```crate/dependencia = "versión del paquete"```

Otra forma de añadir ```rand``` cómo dependencia es ejecutando el siguiente comando:
```cargo add rand```

Es importante mantener estas versiones al ejecutar/usar estos ejemplos, debido que a mayores versiones, más cambios pueden existir, lo que podría romper alguna parte del código existente.

Una forma de tener las correciones de errores pero sin romper el código es no usando ```0.8.5``` sino ```^0.8.5```, si existe una versión con correción de errores automáticamente se actualiza el paquete, pero si existe una versión que modifica el paquete con features o cosas por el estilo (cómo una versión ```0.9.0``` en adelante) entonces el paquete no se actualiza.

Para saber más sobre todo esto, deberías leerlo en el libre gratuito y ofícial de Rust: [Aquí](https://book.rustlang-es.org/ch02-00-guessing-game-tutorial#usando-un-crate-para-obtener-más-funcionalidad)


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

<a id="bib_crate1"></a>

1. [¿Qué es crate en Rust?](https://localhorse.net/article/que-es-crate-en-rust#:~:text=Un%20crate%20en%20Rust%20es%20la%20unidad,dependencias%20y%20reutilizar%20código%20de%20forma%20eficiente.)

<a id="bib_crate2"></a>

2. [Paquetes y Crates](https://book.rustlang-es.org/ch07-01-packages-and-crates#paquetes-y-crates)
