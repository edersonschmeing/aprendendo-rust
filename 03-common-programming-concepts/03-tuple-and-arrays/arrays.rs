use std::mem;


fn main() {

    let mut a: [i32; 5]/*:[i32;5]*/ = [1, 2, 3, 4, 5];

    println!("a has {} elements, first is {}", a.len(), a[0]);
    a[0] = 321;
    println!("first value of a is {}", a[0]);

    assert_eq!(a, [321, 2, 3, 4, 5]);

    if a != [1, 2, 3, 5, 6]
    // size must match
    {
        println!("arrays not equal!");
    }

    // fill an array with 1s
    let b = [1u16; 10]; // try changing to 5
    for i in 0..b.len() {
        println!("{}", b[i]);
    }

    // just print the entire arary
    println!("{:?}", b);
    println!("b took up {} bytes", mem::size_of_val(&b));

    // multidimensional array
    let matriz: [[f32; 3]; 3] = [[1.0, 0.0, 0.0], 
                                 [2.0, 2.0, 1.0],
                                 [0.0, 2.0, 1.0]];
    println!("{:?}", matriz);

    // print all the diagonal values
    for i in 0..matriz.len() {
        for j in 0..matriz[i].len() {
            if i == j {
                println!("matriz[{}][{}] = {}", i, j, matriz[i][j]);
            }
        }
    }
}