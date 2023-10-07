//// 2023/10/07 // 14:53 //

//// #17 Errores

// Errores

//Todos los programadores lo sabemos, a veces, suceden cosas que no esperamos
// en nuestro código y no hay nada que podramos hacer al respectro.

// En estos casos, Rust nos ofrece la macro panic!. Cuando se ejecuta la macro
// panic!, el programa imprimirá un mensaje de fallo, se desenredará y limpiará la
// pila y luego se cerrará.

// De manera predeterminada, cuando ocurre un pánico, el programa comienza a
// desenredarse, lo que significa que Rust vuelve a la pila y limpia los datos de
// cada función que encuentra.

// Obivamente, desenredarse y limpiar es mucho trabajo. La alternativa es abortar
// inmediatamente, lo que finaliza el programa sin limpiarlo. El sistema operativo
// deberá limpiar la memoria que estaba usando el programa.

// Si en su proyecto necesita hacer que el binario resultante sea lo más pequeño
// posible, puede cambiar de desenredar a abortar en caso de pánico.

// use std::fs::File;
// use std::io::ErrorKind;
// use std::io::{self, Read};


// use std::fs;
// use std::io;


// use std::fs::File;


use std::fs::File;
use std::error::Error;


// fn main() {

//     // panic!("error");


//     // let vector = vec![100,200,300,400,500];

//     // let aux = vector[9];


//     // $env:RUST_BACKTRACE=1; cargo run


//     //// Errores recuperables con Result
//     // La mayoría de los errores no son lo sificiente graves como para
//     // requerir que el programa se detenga por completo.
//     // A veces, cuando una función falla, es por una razón que podemos
//     // valorar y responder fácilmente.
//     //
//     // Por ejempto, si intentamos abrir un archivo y la operación falla porque el
//     // archivo no existe, es posible que deseemos crear el archivo
//     // en lugar de finalizar el proceso.
//     ////

//     // // Enumeración Result
//     // enum Result<T, E> {
//     //     Ok(T),
//     //     Err(E),
//     // }


//     // let f = File::open("hola.txt"); // no type
//     // let f: u32 = File::open("hola.txt"); // error

//     // let f = File::open("hola.txt");
    
//     // let f = match f {
//     //     Ok(fichero) => fichero,
//     //     Err(error) => panic!("Error abriendo el fichero: {:?}", error),
//     // };
//     // // $env:RUST_BACKTRACE=0


//     // let f = File::open("hola.txt");

//     // let f = match f {
//     //     Ok(fichero) => fichero,
//     //     Err(error) => match error.kind() {
//     //         ErrorKind::NotFound => match File::create("hola.txt") {
//     //             Ok(fichero_creado) => fichero_creado,
//     //             Err(e) => panic!("Error creando el fichero: {:?}", e),
//     //         },
//     //         other_error => {
//     //             panic!("Error abriendo el fichero: {:?}", other_error)
//     //         }
//     //     }
//     // };


//     // let f= File::open("hola.txt").unwrap();


//     // let f = File::open("hola.txt").expect("Error al abrir hola.txt");


//     // let f = File::open("hola.txt")?;
    
// }

fn main() -> Result<(), Box<dyn Error>> {
    
    let f = File::open("hola.txt")?;
    Ok(())

}


// fn ultimo_caracter_de_primera_linea(texto: &str) -> Option<char> {
//     texto.lines().next()?.chars().last()
// }


// fn leer_usuario_desde_fichero() -> Result<String, io::Error> {
//
//     fs::read_to_string("hola.txt")
//
// }


// fn leer_usuario_desde_fichero() -> {
//     let mut f = File::open("hola.txt");
//     let mut s = String::new();
//     f.read_to_string(&mut s)?;
//     Ok(s)
// }


// fn leer_usuario_desde_fichero() -> Result<String, io::Error> {
//     let f = File::open("hola.txt");
//
//     let mut f = match f {
//         Ok(fichero) => fichero,
//         Err(e) => return Err(e),
//     };
//
//     let mut s = String::new();
//
//     match f.read_to_string(&mut s) {
//         Ok(_) => Ok(s),
//         Err(e) => Err(e),
//     }
// }
 