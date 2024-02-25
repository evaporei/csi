use std::fs::{self, File, OpenOptions};
use std::io::{prelude::*, self, BufRead, SeekFrom};
use std::path::Path;

#[repr(u8)]
enum Tag {
    Table,// TupleList
    Tuple,
    String,
}

struct Tuple(Vec<u8>);

struct Row(Vec<String>);

impl From<Row> for Tuple {
    fn from(row: Row) -> Self {
        let mut tuple = vec![];

        tuple.write(&[Tag::Tuple as u8]).unwrap();
        tuple.write(&row.0.len().to_le_bytes()).unwrap();

        for value in row.0 {
            tuple.write(&value.len().to_le_bytes()).unwrap();
            tuple.write(&value.into_bytes()).unwrap();
        }

        Self(tuple)
    }
}

fn main() {
    let mut f = OpenOptions::new()
        .read(true)
        .write(true)
        .open("test.mf")
        .unwrap();
    f.seek(SeekFrom::Start(0)).unwrap();
    f.write(&[Tag::Table as u8]).unwrap();
    f.seek(SeekFrom::Start(1)).unwrap();
    let tuple: Tuple = Row(vec![]).into();
    f.write(&tuple.0).unwrap();

    // TLV = Tag/Type, Length, Value
    // id, name, age
    // "50", "isabella", "24"
    // 01 01 02 030250 0308isabella 030224 02 030250 0308isabella 030224
}
