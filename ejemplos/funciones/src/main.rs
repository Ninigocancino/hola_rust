fn main() {

    println!("Hello, world!");

    // mi_funcion();

    funcion_dos(7);

    let i: i8 = funcion_tres();

    println!("El valor se i es: {}",i);

}

/* 
fn mi_funcion() {

    println!("Está es mi función");
}
*/

fn funcion_dos(i: i32) {

    println!("El valor de i: {}", i);
}

fn funcion_tres() -> i8{

    13
}