enum Direcao {
    Norte,
    Sul,
    Leste,
    Oeste,
}

fn mover(d: Direcao) {
    match d {
        Direcao::Norte => println!("Indo para o Norte!"),
        Direcao::Sul => println!("Indo para o Sul!"),
        Direcao::Leste => println!("Indo para o Leste!"),
        Direcao::Oeste => println!("Indo para o Oeste!"),
    }
}

fn main() {
    let direcao = Direcao::Leste;
    mover(direcao);
}