fn main() {

    // Las variables en Rust son fuertemente tipadas, lo que significa que debemos indicar el tipo de datos que contendrá la variable

    /*Las variables númericas en Rust pueden contener los siguientes datos:
    
    ----LONGITUD-------CON SIGNO-------SIN SIGNO----
    |   8-bits     |   i8          |   u8          |
    |   16-bits    |   i16         |   u16         |
    |   32-bits    |   i32         |   u32         |
    |   64-bits    |   i64         |   u64         | 
    |   128-bits   |   i128        |   u128        |
    
     */
    



    /* 
    //Ejemplo de variable con una extensión de 8 bits y sin signo

    let edad: u8 = 25;


    //Ejemplo de variable que guarda valor string

    let nombre: &str = "Goku"; 


    //Mostrar en consola

    println!("Hola soy {nombre} y tengo {edad}");

    */



    //En Rust las variables por defecto son inmutables, por lo que una vez declaradas no pueden cambiar de valor. Si necesitamos que la variable creada pueda cambiar de valor durante las líneas posteriores nucesitamos agregar la palabra reservada 'mut' desúés de la palabra reservada 'let'



//-----------------------------------------RECEPCIÓN DE DATOS DEL USUARIO-----------------------------------------------
     
    println!("Por favor introduce tu nombre");

    let mut nombre: String = String::new(); //Esta es una variable vacia

    std::io::stdin().read_line(&mut nombre).unwrap();
    nombre = nombre.trim().to_string();

    println!("Por favor introduce tu edad: ");
    let mut edad : String = String::new();
    std::io::stdin().read_line(&mut edad).unwrap();

    let edad_int : u8 = edad.trim().parse().unwrap();

    println!("Hola, bienvenido {} de {} años", nombre, edad_int);

    /* 

    let x = 3;
    println!("El valor de 'x' es igual a: {}", x);

    */


    /* 


    //Reasignación de valores en una variable

    let mut x = 3;
    println!("El valor de 'x' es igual a: {}", x);

    x = 7;
    println!("El valor de 'x' ahora es igual a: {}", x);

    */
}
