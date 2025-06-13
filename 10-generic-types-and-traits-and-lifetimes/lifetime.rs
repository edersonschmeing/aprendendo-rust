//Adicionamos uma anotação de lifetime explícita 'a,
// dizendo que as referências de entrada e a de saída 
//vivem pelo menos o mesmo tempo 'a:
fn maior<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

fn exemplo01() {
    let string1 = String::from("maçã");
    let string2 = String::from("banana");

    let resultado = maior(&string1, &string2);
    println!("Maior: {}", resultado);

}

struct Pessoa<'a> {
    nome: &'a str,
}

fn exemplo02() {
    let nome = String::from("João");
    let p = Pessoa { nome: &nome };

    println!("Pessoa: {}", p.nome);
}

fn main() {
    
    exemplo01(); 

    exemplo02(); 

}

