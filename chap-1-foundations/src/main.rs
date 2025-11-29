use std::cell::{Cell, RefCell};

fn main() {
    let mut s = Box::new(42);
    dbg!(&s);
    replace_with_84(&mut s);
    dbg!(&s);

    let x = RefCell::new(42);
    dbg!(&x);
    dbg!(x.replace(84));
    dbg!(x);

    let z = std::sync::atomic::AtomicU32::new(0);
    dbg!(z.fetch_add(3, std::sync::atomic::Ordering::Relaxed));
    dbg!(z);

    let mut c = Cell::new(42);
    dbg!(&c.get());
    c.set(84);
    dbg!(c.get());
    dbg!(c.replace(33));
    dbg!(c.get());
    let foo = c.get_mut();
    dbg!(&foo);
    *foo = 55;
    dbg!(foo);
    dbg!(c);
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
