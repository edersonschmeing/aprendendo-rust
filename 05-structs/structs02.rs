struct Retangulo {
    largura: u32,
    altura: u32,
}

impl Retangulo {
    fn area(&self) -> u32 {
        self.largura * self.altura
    }

    fn eh_quadrado(&self) -> bool {
        self.largura == self.altura
    }
}

fn main() {
    let r = Retangulo { largura: 30, altura: 50 };
    println!("Área: {}", r.area());
    println!("É quadrado? {}", r.eh_quadrado());
}
