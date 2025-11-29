fn main() {
    let mut x = Box::new(42);
    let mut z = &x;
    for i in 0..100 {
        dbg!(z);
        x = Box::new(i);
        z = &x;
    }
    dbg!(z);
}
