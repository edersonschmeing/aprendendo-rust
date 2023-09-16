pub fn demo_if_statement() {
    let temp = 35;

    if temp > 30 {
        // curly braces are mandatory!
        println!("really hot outside!");
    } else if temp < 10 {
        println!("really cold, don't go out!");
    } else {
        println!("temperature is OK");
    }

    // if is an expression!
    let day = if temp > 20 { "sunny" } else { "cloudy" };
    println!("today is {}", day);

    // 20+ hot, <20 cold
    println!(
        "it is {}",
        if temp > 20 {
            "hot"
        } else if temp < 10 {
            "cold"
        } else {
            "OK"
        }
    );

    println!(
        "it is {}",
        if temp > 20 {
            if temp > 30 {
                "very hot"
            } else {
                "hot"
            }
        } else if temp < 10 {
            "cold"
        } else {
            "OK"
        }
    );
}


fn demo_declaracoes_if() { 

    let temperatura = 35; 

    if temperatura > 30 {
  
        println!("Muito quente lá fora!");
  
    } else if temperatura < 10 {
  
        println!("Muito frio, não saia!");
  
    } else {
  
        println!("Temperatura está agradável.");
  
    }

    let dia = if temperatura > 20 { "ensolarado" } else { "nublado" };
    println!("O dia esta {}.", dia);
 
    println!(
        "Está {}.",
        if temperatura > 20 {
            "quente"
        } else if temperatura < 10 {
            "frio"
        } else {
            "agradável"
        }
    );

    println!(
        "Está {}",
        if temperatura > 20 {
            if temperatura > 30 {
                "muito quente"
            } else {
                "quente"
            }
        } else if temperatura < 10 {
            "frio"
        } else {
            "agradável"
        }
    );
}