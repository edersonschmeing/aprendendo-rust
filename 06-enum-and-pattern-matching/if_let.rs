fn main() {
    let cor_preferida = Some("Azul");

    // Só entra no bloco se for Some(valor)
    if let Some(cor) = cor_preferida {
        println!("Cor preferida é: {}", cor);
    } else {
        println!("Sem cor preferida");
    }

    let nenhuma_cor: Option<&str> = None;

    if let Some(cor) = nenhuma_cor {
        println!("Cor: {}", cor);
    } else {
        println!("Nenhuma cor definida");
    }
}