fn main() {
    let mut pilha = vec![1, 2, 3];

    // .pop() retorna Option<T>
    while let Some(valor) = pilha.pop() {
        println!("Retirado da pilha: {}", valor);
    }



    let texto = "Rust";
    let mut chars = texto.chars();

    while let Some(c) = chars.next() {
        println!("Caractere: {}", c);
    }
}