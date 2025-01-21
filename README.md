# guess-and-learn

Todo este contenido está basado en el libro online oficial de Rust y en el curso de Microsoft

y

Mis modificaciones y mis gustos 


## Índice de contenidos

* [Código inicial](#item_init)
* [Segundo código](#item_second)
* [Tercer código](#item_third)
* [Sobre usar o no ```expect```](#item_expect)
* [Librería std](#item_std)


<hr>

## Juego de adivinanzas en Rust, para además aprender

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

1. ```use std::io;``` esta parte es fácil, sólo hay que recordar que ```std``` es la biblioteca "standard" de Rust, e ```io``` es la biblioteca de entrada y salida de dentro de "standard" (io = input output). 
Esta biblioteca sirve para que el usuario imprima y/o lea datos.

2. ```let mut adivina = String::new();``` se crea una variable "string" vacía pero mutable (```String::new``` crea una instancia de ```String```).

3. ```io::stdin()``` llama a ```stdin``` de dentro del modulo "io".
    Stdin es un tipo que representa un "manejador" de la entrada estandar para un terminal.

4. Luego viene ```.read_line(&mut adivina)```, que es parte de ```stdin()```.
    
    ```.read_line(&mut adivina)``` llama al método read_line (un "manejador" de entrada del usuario) y pasamos cómo parametro la variable ```adivina``` (referencia de ella, pero mutable) antes creada, esto es para decirle en la cadena que se debe almacenar lo que el usuario escriba.

    Osea que, lo que hace esta línea es: darle una cadena mutable a un método, el cual añadirá lo que escriba el usuario a esta cadena.
    
    Tanto la parte del "read line" cómo la siguiente son hablando de lógica, la misma línea (todo podría ir en la misma línea de código Rust, pero por simplicidad se separa en tres).
    La línea de "read line" recibe un argumento, pero en este caso, también devuelve otro, y lo que devuelve es un [```enum```][enum] llamado ```Result``` (tengo una breve expliación de lo que es un enum [acá](https://github.com/FabrizioJordan/hello-cargo/tree/main?tab=readme-ov-file#loop-bucle-infinito))

    ```Result``` tiene dos posibles variantes ```Ok``` y ```Err``` (error).
    Esto es parecido a JS cuando se llama a una API, la llamada puede ser exitosa (```Ok```) o puede ser fallida (```Err```).
    El error y tipo de error junto con toda su información se almacenan en la variable ```Err```, mientras que en ```Ok``` se almacena el valor generado existosamente.

5. El tipo ```Result``` al igual que otros tipos, tienen métodos predefinidos. 

    Uno de estos métodos se llama ```expect```, el cual sirve para que si ```Result``` devuelve ```Err``` entonces se ejecute el código dentro de ```expect```. (si no logras entender ```Result```, ve el final del documento)

    Osea que ```.expect("No se pudo leer la línea");```, sirve básicamente cómo un ```catch``` del ```try-catch```, el cual si el código dentro de ```try``` da un error ```catch``` devuelve un código o imprime en pantalla un error para ayudar al usuario.

Esta explicación finalizó.


## Segundo juego, con números random

<a id="item_second"></a>

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


Acá te dejo el código de la función ```segunda_adivinanza```:

```
use std::io;
use rand::Rng;

fn main() {
    segunda_adivinanza();
}

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
```

Te explico el código:

```use rand::Rng;``` acá se está llamando a la dependencia ```rand``` y a su trait llamado ```Rng```, ¿que es un trait?, un trait es algo que va a ayudar a nuestro compilador a entender nuevos tipos y sus métodos asociados, los cuales no vienen con Rust por defecto. Más sobre los traits [acá](https://book.rustlang-es.org/ch10-02-traits)

Ahora vamos a la línea ```let azar_numero = rand::thread_rng().gen_range(1..=100);```.

Lo que hace ```rand::thread_rng()``` es una función que nos permite generar los números aleatorios, y ```.gen_range(1..=100)``` es el método para generar los números con un rango específico. "gen range", osea generador de rangos sirve así; ```start..=end``` o ```inicio..=final``` pero usando los números que sirvan.

Todo esto se guarda en la variable ```azar_numero```.

Ya terminamos el segundo ejemplo, vamos con el tercero.


## Tercer juego, uno más real


<a id="item_third"></a>

Ahora vamos a no mostrar el número a adivinar, sino que vamos a verificar si el número que escribió el usuario es el mismo que el número a adivinar.

Comencemos con el tercero ejemplo.

```
use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    primer_adivinanza();
    segunda_adivinanza();
    // tercer_adivinanza();
}

fn tercer_adivinanza(){
    // este código no se ejecutará, está incompleto

    println!("Elegiste el número: {guess}");

    match guess.cmp(&num_secreto) {
        Ordering::Greater => println!("Muy grande!"),
        Ordering::Less => println!("Muy pequeño!"),
        Ordering::Equal => println!("Ganaste! El número era: {} ", &num_secreto),
    }
}
```

Te dejo una explicación de la parte importante:

```use std::cmp::Ordering;``` básicamente llama a la biblioteca estándar para que traiga ```Ordering``` (otro enum) desde ```cpm```, ```Ordering``` tiene tres variantes; ```greater```, ```less``` y ```equal```, que son los tres posibles resultados de una posible comparación númerica.

Ahora ¿Que hace la siguiente línea?

```
match guess.cmp(&num_secreto) {
    Ordering::Greater => println!("Muy grande!"),
    Ordering::Less => println!("Muy pequeño!"),
    Ordering::Equal => println!("Ganaste! El número era: {} ", &num_secreto),
}
```

Cómo ya deberías saber, ```match``` es cómo un switch en JS, es un controlador de flujos que permite comparar un valor con patrones y ejecutar código ante una comparación exitosa.

Entonces lo que se hace es básicamente;

Se toma una referencia del número secreto y el número dentro de la variable ```guess```, los cuales se comparan entre sí gracias al método ```cmp```.

Luego se devulve una variante del enum ```Ordering``` (p.e. ```less```) el cual se comparará con lo impuesto en nuestro ```match```, osea que se compara con ```guess```. 

Hay que recordar que lo devuelto es una variante que nos dirá si el número adivinado es mayor, menor o igual que el número secreto.


Terminada esta explicación, vamos a completar el código.

```
use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    tercer_adivinanza();
}

fn tercer_adivinanza(){
    let num_secreto = rand::thread_rng().gen_range(1..=100);

    println!("Por favor, escríbe el número: ");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("No se pudo leer la línea");

    let guess: u32 = guess.trim().parse().expect("Por favor, escríbe un número");
    
    println!("Elegiste el número: {guess}");
    
    match guess.cmp(&num_secreto) {
        Ordering::Greater => println!("Muy grande!"),
        Ordering::Less => println!("Muy pequeño!"),
        Ordering::Equal => println!("Ganaste!"),
    }
    
    println!("El número era: {} ", &num_secreto)
}
```

Por fin nuestro adivinanza está completa.

Acá te dejo algunas explicaciones:

Línea ```let guess: u32 = guess.trim().parse().expect("Por favor, escríbe un número");``` :

Primero se modifica una variable de tipo int sin signo y de 32 bits (hay que recordar que esta variable ya fue creada anteriormente, ahora está sufriendo una modificación en su valor, lo que se llama [*Shadowing*][var_shadow]).

Luego se le asigna el dato de la variable antigua pero con modificaciones (```let guess: u32 = guess...```), ```.trim()``` le saca los espacios tanto iniciales como finales, luego ```.parse()``` lo convierte de un tipo "String" a un tipo ```u32```. Hay que recordar que para convertir de un tipo num a un tipo ```String``` la variable a convertir no puede tener ni espacios ni nada que no sea un número (a menos que el número sea con coma, etc).

Y por último viene ```.expect("Por favor...")``` el cual le dice al programa que luego de todas las modificaciones lo esperable es que el dato dentro de la variable sea un "integer" y no un tipo ```String```, si no funciona entonces se debe parar el programa y mostrar al usuario el texto de dentro del ```expect```. 

Básicamente ```.parse()``` devuelve lo mismo que ```Result```, o un ```Err``` o un ```Ok```, los cuales nos dicen si en este caso la conversión fue errónea o exitosa.


Fín de este código, acá te dejo un último.

## Último juego, en bucle y entrada no válida

Primero vamos a añadirle un bucle:

```
use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("\n");
    println!("Bienvenido a este juego de adivinanza");
    println!("\n");

    ultima_adivinanza();
}
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
```

Este cambios es fácil, sólo se mete todo el contenido de la función en un ```loop```, ¿se podría haber hecho de otra forma? Sí, podríamos simplemente llamar desde la función ```main``` a la función ```ultima_adivinanza``` dentro de un loop, pero de esa forma el código no estaría implementado directamente desde la función, lo que ahora no nos sirve.

Que es un ```loop``` lo aprendés [acá][what_is_loop] o te dejo [mi expliación con código][loop_by_me].

Ahora le vamos a hacer una pequeña modificación a nuestro código, haremos que cuando el usuario acierte el número, el programa se cierre.

Vamos a modificar solo nuestro ```match```:

```
match guess.cmp(&num_secreto) {
    Ordering::Greater => println!("Muy grande!"),
    Ordering::Less => println!("Muy pequeño!"),
    Ordering::Equal => {
        println!("Ganaste!");
        println!("El número ganador fue: {} ", &num_secreto);
        break;
    }
}
```

Ahora se dejó de solo tener un mensaje de ```Ganaste!```, ahora también se le compartirá el número ganador y espcecialmente se cerrará el programa con un ```break;```.

Todo esto se encierra dentro de una función debido a que ahora se usará más de una línea.


Ahora te voy a explicar el nuevo siguiente código:

Este nuevo código nos va a permitir chequear si lo escrito por el usuario es válido.

```
//let guess: u32 = guess.trim().parse().expect("Por favor, escríbe un número");

let guess: u32 = match guess.trim().parse() {
    Ok(num) => num,
    Err(_) => {
        println!("Error número no válido.");
        println!("Escríba un número válido.");
        continue;
    },
};
```

El código comentado (el que empieza con ```//```) es el código que anteriormente usabamos.

Ahora cambiamos ```expect``` por un ```match```.

Esto es para poder manejar posibles errores existentes, cómo que el número escrito por el usuario no sea considerado válido por el programa para poder ser convertido a ```u32```, aunque también nos permitirá manejar cualquier error más allá de una conversión fallida.

Cambiamos de ```expect``` a ```match``` para así no bloquear el programa entero, esto nos sirve debido a que es un programa en bucle.

¿Porqué usamos ```Ok(num)``` y ```Err(_)```?

Responder esto nos devulve para atrás, donde decíamos que ```.parse()``` devolvía ```Ok``` y ```Err```.

En el caso exitoso de conversión tan solo hacemos ```Ok(num) => num``` para que el número que ```Ok``` nos devuelve sea pasado al siguiente método.

Si la conversión falló entonces usaremos:

```
Err(_) => {
    println!("Error número no válido.");
    println!("Escríba un número válido.");
    continue;
}
```

```Err(_)``` nos dice que todo error se manejará con la función siguiente ```=> { ... }```.

Se usa ```_``` dentro de ```Err``` para decirle al programa que todo error será manejado y no sólo un único caso.

¿¿Porqué todo esa función y no solo un simple ```continue,```??

Es simple, si queremos que el usuario sepa que está cometiendo un error entonces tenemos que hacerselo saber.

## Más sobre lo visto


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


[enum]: https://book.rustlang-es.org/ch06-00-enums  "Enums y Pattern Matching"

[prelude]: https://doc.rust-lang.org/std/prelude/index.html  "Module prelude"

[var_shadow]: https://book.rustlang-es.org/ch03-01-variables-and-mutability#shadowing "Shadowing"

[what_is_loop]: https://doc.rust-lang.org/rust-by-example/flow_control/loop.html "loop"

[loop_by_me]: https://github.com/FabrizioJordan/hello-cargo/tree/main?tab=readme-ov-file#loop-bucle-infinito "Loop: Bucle infinito."

<a id="bib_crate1"></a>

1. [¿Qué es crate en Rust?](https://localhorse.net/article/que-es-crate-en-rust#:~:text=Un%20crate%20en%20Rust%20es%20la%20unidad,dependencias%20y%20reutilizar%20código%20de%20forma%20eficiente.)

<a id="bib_crate2"></a>

2. [Paquetes y Crates](https://book.rustlang-es.org/ch07-01-packages-and-crates#paquetes-y-crates)