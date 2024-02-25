use std::fs::OpenOptions;
// use std::fs::{self, File};
use std::io::prelude::*;
// use std::io::{self, BufRead, SeekFrom};
// use std::path::Path;
use std::os::unix::fs::FileExt;

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
        tuple.write(&(row.0.len() as u16).to_be_bytes()).unwrap();

        for value in row.0 {
            tuple.write(&[Tag::String as u8]).unwrap();
            tuple.write(&(value.len() as u16).to_be_bytes()).unwrap();
            tuple.write(&value.into_bytes()).unwrap();
        }

        Self(tuple)
    }
}

fn main() {
    let mut f = OpenOptions::new()
        .read(true)
        .write(true)
        .truncate(true) // TODO: remove me
        .open("test.mf")
        .unwrap();
    // f.seek(SeekFrom::Start(0)).unwrap();
    f.write(&[Tag::Table as u8]).unwrap();
    f.write(&1u16.to_be_bytes()).unwrap();
    // f.seek(SeekFrom::Start(1)).unwrap();
    // let tuple: Tuple = Row(vec![]).into();
    let tuple: Tuple = Row(vec!["a".into(), "b".into(), "c".into()]).into();
    f.write(&tuple.0).unwrap();

    let mut stuff = [0; 18];
    f.read_at(&mut stuff, 0).unwrap();
    dbg!(stuff);

    // TLV = Tag/Type, Length, Value
    // id, name, age
    // "50", "isabella", "24"
    // 01 01 02 030250 0308isabella 030224 02 030250 0308isabella 030224
}

// 00 - table
// 0001 - one entry
// 01 - tuple
// 0003 - three entries
// 02 - string
// 0001 - length one
// 61
// 02 - string
// 0001 - length one
// 62
// 02 - string
// 0001 - length one
// 63

// 0000000 0000 0101 0003 0200 0161 0200 0162 0200
// 0000010 0163                                   
// 0000012

// 00 - table
// 0001 - one entry
// 01 - tuple
// 0003 - three entries
// 02 - string
// 0001 - length one
// 61 - "a"
// 02 - string
// 0001 - length one
// 62 - "b"
// 02 - string
// 0001 - length one
// 63 - "c"

// 0000000 0000 0101 0300 0002 6101 0002 6201 0002
// 0000010 6301                                   
// 0000012

// [src/main.rs:53] stuff = [
//     0,   // table
//     0, 1,// one entry
//     1,   // tuple
//     0, 3,// three entries
//     2,   // string
//     0, 1,// length
//     97,  // "a"
//     2,   // string
//     0, 1,// length
//     98,  // "b"
//     2,   // string
//     0, 1,// length
//     99,  // "c"
// ]
