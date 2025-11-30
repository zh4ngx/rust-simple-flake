fn main() {
    struct MutStr<'a, 'b> {
        s: &'a mut &'b str,
    }
    let mut s = "hello";
    *MutStr { s: &mut s }.s = "world";
    dbg!(s);
}
