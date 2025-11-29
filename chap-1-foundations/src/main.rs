fn main() {
    let my_doc = "hi, i think i have a delimiter";
    let my_delimiter = ",";
    let x = &mut StrSplit {
        document: my_doc,
        delimiter: my_delimiter,
    };
    x.next();
    dbg!(x.document);
}

struct StrSplit<'s, 'p> {
    delimiter: &'p str,
    document: &'s str,
}

impl<'s, 'p> Iterator for StrSplit<'s, 'p> {
    type Item = &'s str;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
fn str_before(s: &str, c: char) -> Option<&str> {
    StrSplit {
        document: s,
        delimiter: &c.to_string(),
    }
    .next()
}
