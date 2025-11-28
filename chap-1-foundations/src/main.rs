fn main() {
    println!("Hello, world!");
    let x = 42;
    let mut y = &x; // y is of type &i32

    println!("y is {y}");
    y = &100;
    println!("y is {y}");
    let z = &mut y;

    *z = &33;
    println!("y is {y}");
}
