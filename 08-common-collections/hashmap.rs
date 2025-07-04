//sequences
//use std::collections::Vec;
//use std::collections::VecDeque;
//use std::collections::LinkedList;

//maps
 //use std::collections::HashMap;
 //use std::collections::BTreeMap;

//Sets
//use std::collections::HashSet;
//use std::collections::BTreeSet;

//Misc
// use std::collections::BinaryHeap;

use std::collections::HashMap;

fn main() {

    let mut shapes = HashMap::new();

    // types that implement the Copy trait are ok
    // types that don't are moved
    let triangle = String::from("triangle");
    shapes.insert(triangle, 3);
    shapes.insert("square".into(), 4);

    //let t = triangle;

    println!("a square has {} sides", shapes["square".into()]);

    // iterate the entire thing
    for (key, value) in &shapes {
        println!("{} : {}", key, value);
    }

    // can overwrite values
    shapes.insert("square".into(), 5);
    println!("{:?}", shapes);

    let e = shapes.entry("square".into());

    // upsert (only insert if it has no value)
    shapes.entry("circle".into()).or_insert(1);
    {
        let actual = shapes.entry("circle".into()).or_insert(2);
        *actual = 0;
    }
    println!("{:?}", shapes);


}