fn main() {
    let mut x = Box::new(42);
    let r = &x;
    if rand() > 0.5 {
        *x = 84;
    } else {
        println!("{}", r);
    }
}

fn rand() -> f64 {
    // Implementation of rand function
    0.4
}

fn replace_with_84(s: &mut Box<i32>) {
    // this is not okay, as *s would be empty:
    // let was = *s;
    // but this is:
    let was = std::mem::take(s);
    // so is this:
    *s = was;
    // we can exchange values behind &mut:
    let mut r = Box::new(84);
    std::mem::swap(s, &mut r);
    assert_ne!(*r, 84);
}
