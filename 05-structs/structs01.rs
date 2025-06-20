struct Ponto {
    x: i32,
    y: i32,
}

fn main() {
    let p = Ponto { x: 10, y: 20 };
    println!("Ponto: ({}, {})", p.x, p.y);
}