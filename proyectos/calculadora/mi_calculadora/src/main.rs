use regex::Regex;

fn main() {
    println!("¡Hola, calculadora!");

    // Expresión regular para sumar dos números
    let re_add = Regex::new(r"(\d+)\s?\+\s?(\d+)").unwrap();

    // Traer datos del usuario
    println!("Por favor introduce tu expresión (ejemplo: 12 + 34): ");
    let mut expression = String::new();
    std::io::stdin().read_line(&mut expression).unwrap();

    // Aplicar la expresión regular
    let caps = re_add.captures(expression.as_str());

    // Verificar si la expresión coincide con el patrón de suma
    if let Some(caps) = caps {
        // Extraer valores de los grupos capturados
        let left_value: i32 = caps.get(1).unwrap().as_str().parse().unwrap();
        let right_value: i32 = caps.get(2).unwrap().as_str().parse().unwrap();

        // Realizar la suma
        let result = left_value + right_value;

        // Mostrar resultados
        println!("Operación: {}", expression.trim());
        println!("Resultado: {}", result);
    } else {
        // Si no coincide con el patrón de suma, mostrar un mensaje de error
        println!("Expresión no válida. Introduce una expresión válida de suma.");
    }
}
