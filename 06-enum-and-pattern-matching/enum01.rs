enum Mensagem {
    Texto(String),
    Numero(i32),
    Coordenada(i32, i32),
    Sair,
}

fn processar(msg: Mensagem) {
    match msg {
        Mensagem::Texto(s) => println!("Texto recebido: {}", s),
        Mensagem::Numero(n) => println!("Número: {}", n),
        Mensagem::Coordenada(x, y) => println!("Coordenadas: ({}, {})", x, y),
        Mensagem::Sair => println!("Encerrando..."),
    }
}

fn main() {
    let msg1 = Mensagem::Texto(String::from("Olá!"));
    let msg2 = Mensagem::Coordenada(10, 20);
    processar(msg1);
    processar(msg2);
}