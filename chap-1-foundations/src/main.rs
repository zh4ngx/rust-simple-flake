fn main() {
    struct MutStr<'a, 'b> {
        s: &'a mut &'b str,
    }
    let mut s = "hello";
    *MutStr { s: &mut s }.s = "world";
    dbg!(s);

    let foo = Foo {
        tiny: true,
        normal: 42,
        small: 1,
        long: 9876543210,
        short: 100,
    };
    dbg!(foo);

    let bar = Foo::default();
    dbg!(bar);
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
struct Foo {
    tiny: bool,
    normal: u32,
    small: u8,
    long: u64,
    short: u16,
}
