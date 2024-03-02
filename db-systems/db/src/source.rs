use crate::fs::{buf_reader, read_lines};
use std::fs::File;
use std::io::{self, BufRead};

// this is lame IMO
pub trait Source: Offset + Metadata {}

pub trait Offset: Iterator<Item = Row> {
    fn offset(&self) -> usize;
}

pub trait Metadata {
    fn table(&self) -> &str;
}

pub struct FileScan {
    offset: usize,
    table: String,
    file: io::BufReader<File>,
}

impl FileScan {
    pub fn new(table: &str) -> Self {
        let mut file = buf_reader(format!("./ml-20m/{table}.csv")).unwrap();
        let mut trash = vec![];
        let offset = file.read_until(b'\n', &mut trash).unwrap();

        Self {
            offset,
            table: table.to_owned(),
            file,
        }
    }
}

impl Offset for FileScan {
    fn offset(&self) -> usize {
        self.offset
    }
}

impl Metadata for FileScan {
    fn table(&self) -> &str {
        &self.table
    }
}

impl Source for FileScan {}

// Tuple
pub type Row = Vec<String>;

impl Iterator for FileScan {
    type Item = Row;

    fn next(&mut self) -> Option<Self::Item> {
        let mut raw = vec![];
        let read = self.file.read_until(b'\n', &mut raw).unwrap();
        if read == 0 {
            return None;
        }
        self.offset += read;

        // TODO: abstract
        Some(
            raw.split(|b| *b == b',')
                .map(|bytes| std::str::from_utf8(bytes))
                .map(Result::unwrap)
                .map(|s| s.trim())
                .map(|s| s.to_owned())
                .collect(),
        )
    }
}

#[derive(Clone, Debug)]
pub struct Schema {
    pub table: String,
    pub fields: Vec<String>,
}

impl Schema {
    pub fn new(table: &str) -> Self {
        Self {
            // omaga
            fields: read_lines(format!("./ml-20m/{table}.csv"))
                .unwrap()
                .next()
                .expect("at least one line in file (the schema)")
                .unwrap()
                .split(',')
                .map(|s| s.to_owned())
                .collect(),
            table: table.to_owned(),
        }
    }
}

/// For now the projection retrieves the fields
/// in the order of the schema (first line in csv).
pub struct Projector<'a> {
    source: &'a mut dyn Iterator<Item = Row>,
    projection: Vec<String>,
    idxs: Vec<usize>,
}

impl<'a> Projector<'a> {
    pub fn new(
        projection: Vec<String>,
        source: &'a mut dyn Iterator<Item = Row>,
        schema: &Schema,
    ) -> Self {
        let idxs = projection
            .iter()
            .filter_map(|p| {
                let opt = schema.fields.iter().position(|f| f == p);

                if opt.is_none() {
                    // for now, comment to disable warning
                    // eprintln!("warn: '{p}' field not found in table '{}'", schema.table);
                }

                opt
            })
            .collect();

        Self {
            idxs,
            source,
            projection,
        }
    }
}

impl<'a> Iterator for Projector<'a> {
    type Item = Row;

    fn next(&mut self) -> Option<Self::Item> {
        let row = self.source.next()?;

        if self.projection.is_empty() {
            return Some(row);
        }

        // we have nothing to do with the described fields
        if self.idxs.is_empty() {
            return None;
        }

        Some(
            row.into_iter()
                .enumerate()
                .filter(|(i, _)| self.idxs.contains(i))
                .map(|(_, s)| s)
                .collect(),
        )
    }
}

pub struct Selector<'a> {
    selection: Vec<String>,
    source: &'a mut dyn Iterator<Item = Row>,
    idx: usize,
    // ugly but it works
    ran: bool,
}

impl<'a> Selector<'a> {
    pub fn new(
        selection: Vec<String>,
        source: &'a mut dyn Iterator<Item = Row>,
        schema: &Schema,
    ) -> Self {
        Self {
            idx: schema
                .fields
                .iter()
                .position(|f| f == &selection[0])
                // we can remove later if we want silent filter (if not found)
                // which can be useful once we have multiple scanners (multi-table queries)
                .expect(&format!(
                    "'{}' field not found in table '{}'",
                    selection[0], schema.table
                )),
            selection,
            source,
            ran: false,
        }
    }
}

impl<'a> Iterator for Selector<'a> {
    // look at me, I'm different o-o
    type Item = Vec<Row>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.ran {
            return None;
        }
        self.ran = true;

        let mut results = vec![];
        while let Some(row) = self.source.next() {
            match &self.selection[1][..] {
                "EQUALS" => {
                    if row[self.idx] == self.selection[2] {
                        results.push(row);
                    }
                }
                _ => {
                    // TODO: should print only on ::new
                    println!(
                        "warn: selection operation '{}' not supported",
                        self.selection[1]
                    );
                }
            }
        }
        Some(results)
    }
}

pub struct NestedJoin<'a> {
    outer: &'a mut dyn Iterator<Item = Row>,
    inner: &'a mut dyn Iterator<Item = Row>,
}

impl<'a> NestedJoin<'a> {
    pub fn new(
        outer: &'a mut dyn Iterator<Item = Row>,
        inner: &'a mut dyn Iterator<Item = Row>,
    ) -> Self {
        Self { outer, inner }
    }
}

impl<'a> Iterator for NestedJoin<'a> {
    type Item = Row;

    // full outer join
    fn next(&mut self) -> Option<Self::Item> {
        match (self.outer.next(), self.inner.next()) {
            (Some(mut outer_row), Some(mut inner_row)) => {
                outer_row.append(&mut inner_row);
                Some(outer_row)
            }
            // to make an inner join, these below should return None
            // however the whole `.next()` would need to consume it all,
            // so the `.collect()` doesn't stop in the middle
            (Some(outer_row), None) => Some(outer_row), // left
            (None, Some(inner_row)) => Some(inner_row), // right
            (None, None) => None,
        }
    }
}
