struct Coordenada {
    x: i32,
    y: i32,
}

impl Coordenada {
    fn nova(x: i32, y: i32) -> Coordenada {
        Coordenada { x, y }
    }
}

fn main() {
    let c = Coordenada::nova(5, 15);
    println!("({}, {})", c.x, c.y);
}
