
//Desestruturação de Struct

struct Ponto {
    x: i32,
    y: i32,
}

fn main() {
    let p = Ponto { x: 7, y: 3 };
    let Ponto { x, y } = p;
    println!("x = {}, y = {}", x, y);
}