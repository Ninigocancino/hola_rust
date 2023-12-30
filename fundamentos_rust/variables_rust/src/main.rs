fn main() {

/* 
    //SINTAXIS BÁSICA DE VARIABLES:

    //En Rust como en otros lenguajes de programación las variables son contenedores y tienen un nombre, un tipo de dato y un valor

    //En Rust, las variables se declaran usando la palabra reservada 'let'. El tipo de datos de la variable puede inferirse del valor que se le asigna, pero también se puede especificar explícitamente.

    // ******EJEMPLO******

    let saludo: &str = "Hola mundo"; //esta línea guarda el valor 'Hola Mundo' que es d etipo string

    println!("{}",saludo); //Se imprime el valor de la variable

*/



 
    //INMUTABILIDAD DE LAS VARIABLES:

    //En Rust las variables son inmutables por defecto, por lo que si se necesita que una variable sea mutable, se debe declarar explicitamente con la palabra reservada 'mut'.


    //******EJEMPLO******


    //Al ejecutar las siguientes lineas se imprimira en consola el valor almacenado en la variable 'x' sin ningún problema

    //let x:i32 = 5;

    //Pero si intentamos sobreescribir un valor a 'x' diferente al declarado la primera vez, se generará un error

    
    //let x:i32 = 8; //Esta línea genera un error pues intenta sobre escribir un valor a la variable 'x' que por defecto es inmutable en Rust


    //println!("{}",x);

    //Para evitar errores al ejecutar varibles a las cuales queremos sore escribir valores más adelante lo hacemos de la siguiente forma:
    
    //let mut y: i32 = 8; //Esta línea asigna el valor inicial a la variable 'y'

    //A diferencia de la variable 'x' la variable 'y' debería ser capaz de aceptar la asignación d eun nuevo valor

    //let y = 12; //Esta línea debe imprimir el nuevo valor que se sobre escribre enla variable 'y'

    //println!("{}",y);



/* 
    //ALCANCE DE LAS VARIABLES:

    //En Rust una variable es válida solo dentro de las llaves {} en las que se declara.

    let variable:&str = "solo se ejecuta dentro de corchetes";

    println!("{}",variable);
*/



/* 
    //SHADOWING

    //Rust permite la "sombra" o redeclaración de variables dentro del mismo ámbito. Esto es útil para cambiar el tipo de dato de una variable sin tener que cambiar su nombre.

    let mut x:i32 = 5;
    x = x + 1;  // Shadowing: x ahora es 6

    println!("{}", x);
*/



}

//println!("{}", variable); //esto genera un error pues está fuera del alcance de la variable

