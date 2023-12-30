fn main() {

    let mut x:u32 = 5;
    println!(" x es igual a: {}", x);

    x = 366999;
    println!("x es igual a: {}", x);

    /* Operaciones */

    println!("Operacioens");

    let suma: i32 = 58 + 67;

    let resta:f64 = 34343.39 -788.3;

    //división entera//
    let multiplicacion:f64 = 445.3 *23.0; 

    //Resto de la divisón
    let resto:i32 = 34 % 4; 

    println!("{}", suma);
    println!("{}", resta);
    println!("{}", multiplicacion);
    println!("{}", resto);

    //Dato tipo booleano

    /*Devuelve valores verdadero o falso */

    println!(" ");
    println!("Booleanos");
    println!(" ");

    let verdadero: bool = true;
    println!("{}", verdadero);

    let falso:bool  = false;
    println!("{}", falso);

    println!(" ");

    //DATO TIPO CARACTER

    println!("Caracter");

    let a:char = 'a';

    let emoji:char = '😁';

    println!("{}", a);
    println!("{}", emoji);
    println!(" ");

    //DATO TIPO COMPUESTO

    //Tupla

    println!("Tuplas");

    let tupla:(i32,u8,i32,f64) = (500,34,87,-344.9);

    let primero: i32 = tupla.0;

    println!("{}", primero);

    println!(" ");

    //Matriz

    println!("Matriz o Array");

    let matriz:[i32;5] = [23,45,2,11,45];

    for elemento in &matriz {
        println!("{}", elemento);
    }


}

