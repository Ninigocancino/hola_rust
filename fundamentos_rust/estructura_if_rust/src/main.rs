fn main() {
    

    // En Rust como en otros lenguajes de programación, la estructura if se utiliza para controlar el flujo de ejecución del programa basándose en una condición booleana (que sea  'True' o 'False'), es decir; podemos usar 'if' para permitir que nuestro programa realizace alguna acción determina, si se cumple cierta condición especifica.

    // Por ejemplo prodríamos hacer que nuestro codigo imprima un texto solo si valor de una variable  'x' es igual a 3

   /* 
    let x = 3;

    if x == 3 {
        println!("El número es 3");
    }
   */

    //  La sintaxis básica de un if en Rust es la siguiente:

    // en Rust, las condiciones en un if deben ser expresiones booleanas. Puedes usar operadores de comparación (<, >, <=, >=, ==, !=) y operadores lógicos (&& para "y", || para "o", ! para "no") para construir condiciones más complejas.

    // Además, en Rust, las condiciones no necesitan paréntesis, pero los bloques de código deben estar rodeados por llaves {}.

    // Por último, ten en cuenta que Rust fomenta el uso de expresiones en lugar de instrucciones. Esto significa que el if puede devolver un valor y asignarse a una variable. Aquí hay un ejemplo:

    let numero = 42;
    let resultado = if numero > 0 { "positivo" } else { "negativo" };
    println!("El número es {}", resultado);
}
