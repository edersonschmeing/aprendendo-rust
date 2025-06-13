fn print_nome(nome: &String) {
    println!("Nome: {}", nome);
}

fn adiciona_exclamacao(nome: &mut String) {
    nome.push('!');
}

//Só pode haver uma referência mutável 
//OU várias referências imutáveis ao mesmo tempo.

fn main() {
    let nome = String::from("Ana");
    print_nome(&nome); // empresta, não move
    println!("Ainda tenho: {}", nome); // tudo ok

    let mut nome = String::from("Ana");
    adiciona_exclamacao(&mut nome); // empresta mutável
    println!("{}", nome); // Ana!


}