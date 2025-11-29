fn main() {
    let my_doc = "hi, i think i have a delimiter";
    let my_delimiter = ",";
    let mut x = StrSplit {
        document: my_doc,
        delimiter: my_delimiter,
    };
    dbg!(&x);
    dbg!(str_before(my_doc, ','));
    dbg!(&x);
    dbg!(x.next());
    dbg!(x.next());
    dbg!(x);
}

#[derive(Debug)]
struct StrSplit<'s, 'p> {
    delimiter: &'p str,
    document: &'s str,
}

impl<'s, 'p> Iterator for StrSplit<'s, 'p> {
    type Item = &'s str;

    fn next(&mut self) -> Option<Self::Item> {
        let chunks: Vec<&str> = self.document.splitn(2, self.delimiter).collect();
        let head = chunks.first().unwrap();
        let tail = chunks.last().unwrap_or(&"");
        self.document = tail;
        Some(head)
    }
}
fn str_before(s: &str, c: char) -> Option<&str> {
    StrSplit {
        document: s,
        delimiter: &c.to_string(),
    }
    .next()
}
