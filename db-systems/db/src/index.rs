use std::io::{BufRead, Seek, SeekFrom};

use crate::btree::BTreeMap;
use crate::fs::buf_reader;
use crate::source::{Row, Schema, Source};

pub struct IndexBuilder<'a> {
    source: &'a mut dyn Source,
    idx: usize,
    ran: bool,
}

impl<'a> IndexBuilder<'a> {
    pub fn new(field: &str, source: &'a mut dyn Source, schema: &Schema) -> Self {
        Self {
            idx: schema
                .fields
                .iter()
                .position(|f| f == field)
                // we can remove later if we want silent filter (if not found)
                // which can be useful once we have multiple scanners (multi-table queries)
                .expect(&format!(
                    "'{}' field not found in table '{}'",
                    field, schema.table
                )),
            source,
            ran: false,
        }
    }
}

impl<'a> Iterator for IndexBuilder<'a> {
    // I'm different too!
    type Item = Index;

    fn next(&mut self) -> Option<Self::Item> {
        if self.ran {
            return None;
        }
        self.ran = true;

        let mut results = BTreeMap::new();
        loop {
            let offset = self.source.offset();
            match self.source.next() {
                Some(row) => {
                    results.insert(row[self.idx].clone(), offset);
                }
                None => break,
            }
        }
        // this is a non sense "btree"
        // results.sort();
        Some(Index::new(results, self.source.table()))
    }
}

type Field = String;

pub struct Index {
    ptrs: BTreeMap<Field, usize>,
    table: String,
}

impl Index {
    fn new(ptrs: BTreeMap<Field, usize>, table: &str) -> Self {
        Index {
            ptrs,
            table: table.into(),
        }
    }

    pub fn search(&self, value: &str) -> Option<Row> {
        let mut file = buf_reader(format!("./ml-20m/{}.csv", self.table)).unwrap();
        file.seek(SeekFrom::Start(*self.ptrs.get(value).unwrap() as u64))
            .unwrap();
        let mut row = vec![];
        let _ = file.read_until(b'\n', &mut row).unwrap();
        // TODO: abstract
        Some(
            row.split(|b| *b == b',')
                .map(|bytes| std::str::from_utf8(bytes))
                .map(Result::unwrap)
                .map(|s| s.trim())
                .map(|s| s.to_owned())
                .collect(),
        )
    }
}
