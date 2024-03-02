use crate::fs::{buf_reader, read_lines};
use std::fs::File;
use std::io::{self, BufRead, Seek, SeekFrom};

// this is lame IMO
pub trait Source: Offset + Metadata + Reset {}

pub trait Offset: Iterator<Item = Row> {
    fn offset(&self) -> usize;
}

pub trait Metadata {
    fn table(&self) -> &str;
}

pub trait Reset {
    fn reset(&mut self);
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

impl Reset for FileScan {
    fn reset(&mut self) {
        self.file.seek(SeekFrom::Start(0)).unwrap();
        let mut trash = vec![];
        self.offset = self.file.read_until(b'\n', &mut trash).unwrap();
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
        assert_eq!(
            selection[1], "EQUALS",
            "SELECTION clause only supports EQUALS"
        );

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
            if row[self.idx] == self.selection[2] {
                results.push(row);
            }
        }
        Some(results)
    }
}

/// Don't ever use this, it just runs forever ;-;
/// I'll see if optimizing the file accesses makes
/// this usable.
pub struct NestedJoin<'a> {
    outer: &'a mut dyn Source,
    inner: &'a mut dyn Source,
    outer_idx: usize,
    inner_idx: usize,
    ran: bool,
}

impl<'a> NestedJoin<'a> {
    pub fn new(
        outer: &'a mut dyn Source,
        inner: &'a mut dyn Source,
        outer_schema: Schema,
        inner_schema: Schema,
        on: Vec<String>,
    ) -> Self {
        assert_eq!(on[1], "EQUALS", "JOIN clauses only supports EQUALS");
        let outer_field = &on[0].split('.').skip(1).next().unwrap();
        let inner_field = &on[2].split('.').skip(1).next().unwrap();
        let outer_idx = outer_schema
            .fields
            .iter()
            .position(|f| f == outer_field)
            .expect("unrecognized field for outer table in JOIN");
        let inner_idx = inner_schema
            .fields
            .iter()
            .position(|f| f == inner_field)
            .expect("unrecognized field for inner table in JOIN");
        Self {
            outer,
            inner,
            ran: false,
            outer_idx,
            inner_idx,
        }
    }
}

impl<'a> Iterator for NestedJoin<'a> {
    // here we go again
    type Item = Vec<Row>;

    // inner join
    fn next(&mut self) -> Option<Self::Item> {
        if self.ran {
            return None;
        }
        self.ran = true;

        let mut results = vec![];
        loop {
            let outer_row = match self.outer.next() {
                Some(row) => row,
                None => break,
            };
            loop {
                let mut inner_row = match self.inner.next() {
                    Some(row) => row,
                    None => break,
                };
                if outer_row[self.outer_idx] == inner_row[self.inner_idx] {
                    let mut result = outer_row.clone();
                    result.append(&mut inner_row);
                    results.push(result);
                }
            }
            self.inner.reset();
        }

        Some(results)
    }
}

pub struct HashJoin<'a> {
    outer: &'a mut dyn Source,
    inner: &'a mut dyn Source,
    outer_idx: usize,
    inner_idx: usize,
    ran: bool,
}

impl<'a> HashJoin<'a> {
    pub fn new(
        outer: &'a mut dyn Source,
        inner: &'a mut dyn Source,
        outer_schema: Schema,
        inner_schema: Schema,
        on: Vec<String>,
    ) -> Self {
        assert_eq!(on[1], "EQUALS", "JOIN clauses only supports EQUALS");
        let outer_field = &on[0].split('.').skip(1).next().unwrap();
        let inner_field = &on[2].split('.').skip(1).next().unwrap();
        let outer_idx = outer_schema
            .fields
            .iter()
            .position(|f| f == outer_field)
            .expect("unrecognized field for outer table in JOIN");
        let inner_idx = inner_schema
            .fields
            .iter()
            .position(|f| f == inner_field)
            .expect("unrecognized field for inner table in JOIN");
        Self {
            outer,
            inner,
            ran: false,
            outer_idx,
            inner_idx,
        }
    }
}

// std's is faster than mine, and it has iterators
use std::collections::BTreeMap;

impl<'a> Iterator for HashJoin<'a> {
    // here we go again
    type Item = Vec<Row>;

    // inner join
    fn next(&mut self) -> Option<Self::Item> {
        if self.ran {
            return None;
        }
        self.ran = true;

        let outer_table = self.outer.into_iter().fold(BTreeMap::new(), |mut acc, row| {
            let outer_column = row[self.outer_idx].clone();
            acc.insert(outer_column, row);
            acc
        });
        let inner_table = self.inner.into_iter().fold(BTreeMap::new(), |mut acc, row| {
            let inner_column = row[self.inner_idx].clone();
            acc.insert(inner_column, row);
            acc
        });

        let mut results = vec![];

        for (key, mut value) in outer_table {
            if inner_table.contains_key(&key) {
                let mut inner_row = inner_table.get(&key).unwrap().clone();
                value.append(&mut inner_row);
                results.push(value);
            }
        }

        Some(results)
    }
}
