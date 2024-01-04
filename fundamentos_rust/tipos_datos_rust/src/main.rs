fn main() {

    //Conocer los tipos de datos que admite Rust es algo fundamental para el aprendizaje  de este  lenguaje
    
    //DATOS DE TIPO PRIMITIVOS

    /*Números enteros con signo:

    Rust admite el uso de numeros enteros con signo de 8,16,32,64 y 132 bits respectivamente que se deben representar de la siguiente forma:

    i8   => Enteros de 8 bits
    i16  => Enteros de 16 bits
    i32  => Enteros de 32 bits
    i64  => Enteros de 64 bits
    i132 => Enteros de 132 bits
    
    Un entero con signo es un tipo de entero que puede representar tanto números positivos como negativos.

     */

    //Ejemplo  de número entero con signo:

    /* 
    let entero:i32 = -42;
    println!("El valor del entero consigno es {}", entero)
    */

    /*Números enteros sin signo :

    Rust admite el uso de numeros enteros sin signo de 8,16,32,64 y 132 bits respectivamente que se deben representar de la siguiente forma:

    u8   => Enteros de 8 bits
    u16  => Enteros de 16 bits
    u32  => Enteros de 32 bits
    u64  => Enteros de 64 bits
    u132 => Enteros de 123 bits

    Un entero sin signo es un tipo de entero que solo puede representar números positivos.
    
     */

    //Ejemplo de datos de tipo entero sin signo:

    /*
    let numero_sin_signo: u32 = 42;
    println!("El valor del entero sin signo es {}", numero_sin_signo)
    */

    //Si usamos un número negativo la línea de código generará un error al ejecutarse:
    
    /* 
    let numero_sin_signo: u32 = -42;
    println!("El valor del entero sin signo es {}", numero_sin_signo);
    */



}
