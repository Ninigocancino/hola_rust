use std::string;

fn main() {
    // Ejercicio 1: 
    //Declara dos variables, a y b, con valores numéricos y realiza operaciones aritméticas básicas (suma, resta, multiplicación, división).
    //Imprime los resultados de las operaciones.

    /* 
    let a:i32 = 12;

    let b:i32 = 2;

    let sum:i32 = a + b;

    let mult:i32 = a * b;

    let div:i32 = a / b;

    let rest:i32 = b - a;

    println!("los resultados son suma: {}, multiplicación {}, división {}, resta {}",sum,mult,div,rest);
    */

    //Declara una variable cadena con un valor de cadena.
    //Convierte la cadena a un número entero y realiza alguna operación matemática con él.
    //Imprime el resultado.

    let cadena:&str = "13";

    let resultado: Result<i32, _> = cadena.parse();

    match resultado {
        Ok(numero) => {
            println!("conversión exitosa. El número es {}", numero);

            let operacion:i32 = numero + numero;
            println!("{}",operacion);


        }
        Err(_) => {
            println!("fallo la conversión");
        }
    }


}
