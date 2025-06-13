fn exemplo01() { 
    let s1 = String::from("Olá");
    let s2 = s1; // s1 é movido para s2, não copiado

    // println!("{}", s1); // Erro! s1 não é mais válido
    println!("{}", s2);
    //Porque String aloca memória na heap, 
    //e Rust quer garantir que só exista um dono responsável por liberar essa memória.
}

fn main() {
    exemplo01()
}
