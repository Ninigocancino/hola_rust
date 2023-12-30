fn main() {

    /* 

    //Estructura if

    let numero: i32 = 13;

    if numero < 10{
        println!("Se cumple la condición");
    } else {
        println!("La condición no se cumple");
    }

    */
    

    
    /* 

    //Estructura "if" "else if"

    if numero < 10{
        println!("el numero es menor que 10");
    } else if numero < 5 {
        println!("El número es menor que 5");
    } else{
        println!("El numero es mayor que 10");
    }

    */


    
     
//------------------------------------------ BUCLES/LOOPS ---------------------------------------------
   
   
//Ejemplo Contador 

    /*

    let mut contador = 0;

    loop{
        println!("Hola!");
        contador+=1;

        if contador == 10{
            break;
        }
    }

    */

//Ejemplo Sumar dos números

    //Variables con valores a sumar:

    let numero_1:i32 = 120;
    let numero_2:i32 = 220;

    let suma: i32 = numero_1 + numero_2;

    loop {
        //Mostrar al usuario los valores a sumar

        println!("Por favor escribe la suma de {} y {}: ", numero_1,numero_2);

        //Obtener del usuario el resultado de la instrucción dada

        let mut usuario_respuesta:String = String::new();
        std::io::stdin().read_line(&mut usuario_respuesta).unwrap();

        let usuario_respuesta_int:i32 = usuario_respuesta.trim().parse().unwrap();

        if usuario_respuesta_int == suma {
            println!("Lo has hecho muy bien el resultado de {} más {} es {}", numero_1,numero_2,suma);
            break;
        } else {
            println!("El resultado {} no es correcto, intenta de nuevo",usuario_respuesta_int);
        }

    }



    /* 

    // while

    let mut contador: i32 = 10;

    while contador != 0{

        println!("Contador: {}", contador);

        contador -= 1;
    }


    //

    let mut matriz = [10,20,30,40,50,60,70,80,90];
    let mut i = 0;

    while i < 9 {
        println!("valor del array en la posicion {}: {}", i, matriz[i]);
        i += 1 ;
    }

    */

/* 
    //Ciclo for

    let matriz = [10,20,30,40,50,60,70,80,90];

    for numero in matriz.iter() {
        println!("valor del elemento es: {}", numero);
    }

*/


}
