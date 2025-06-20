struct Telefone {
    ddi: u8,
    ddd: u8,
    numero: u32,
}    

struct Pessoa {
    idade: u8,
    telefone: Telefone,
}

fn main() {
    let t = Telefone { ddi: 55, ddd: 45, numero: 999703031};
    let p = Pessoa { idade: 30, telefone: t };

    println!(
        "Idade: {}, Telefone: {}-{}-{:?}",
        p.idade,
        p.telefone.ddi,
        p.telefone.ddd,
        p.telefone.numero
    );
}
