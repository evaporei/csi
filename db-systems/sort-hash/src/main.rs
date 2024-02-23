use std::fs::{self, File};
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

type Parts = Vec<String>;

#[derive(Debug, Default)]
struct Query {
    projection: Option<Parts>, // fields/attributes
    selection: Option<Parts>,  // conditions
    scan: Option<Parts>,       // tables
}

use serde_json::Value;

impl From<Value> for Query {
    fn from(json: Value) -> Self {
        let mut query = Query::default();
        for clause in json.as_array().unwrap() {
            if clause[0] == "PROJECTION" {
                // lol
                query.projection = Some(
                    clause[1]
                        .as_array()
                        .unwrap()
                        .into_iter()
                        .map(|a| a.as_str().unwrap().to_owned())
                        .collect(),
                );
            }
            if clause[0] == "SELECTION" {
                // lol
                query.selection = Some(
                    clause[1]
                        .as_array()
                        .unwrap()
                        .into_iter()
                        .map(|a| a.as_str().unwrap().to_owned())
                        .collect(),
                );
            }
            if clause[0] == "SCAN" {
                // lol
                query.scan = Some(
                    clause[1]
                        .as_array()
                        .unwrap()
                        .into_iter()
                        .map(|a| a.as_str().unwrap().to_owned())
                        .collect(),
                );
            }
        }
        query
    }
}

use std::iter::Skip;

struct FileScan {
    table: String,
    lines: Skip<io::Lines<io::BufReader<File>>>,
}

impl FileScan {
    fn new(table: &str) -> Self {
        Self {
            table: table.to_owned(),
            lines: read_lines(format!("./ml-20m/{table}.csv")).unwrap().skip(1),
        }
    }

    fn table(&self) -> &str {
        &self.table
    }
}

// Tuple
type Row = Vec<String>;

impl Iterator for FileScan {
    type Item = Row;

    fn next(&mut self) -> Option<Self::Item> {
        let raw = self.lines.next()?.ok()?;

        Some(raw.split(',').map(|s| s.to_owned()).collect())
    }
}

struct Schema {
    table: String,
    fields: Vec<String>,
}

impl Schema {
    fn new(table: &str) -> Self {
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

struct Projector<'a> {
    source: &'a mut dyn Iterator<Item = Row>,
    projection: Vec<String>,
    idxs: Vec<usize>,
}

impl<'a> Projector<'a> {
    fn new(
        projection: Vec<String>,
        source: &'a mut dyn Iterator<Item = Row>,
        schema: Schema,
    ) -> Self {
        Self {
            idxs: projection
                .iter()
                .map(|p| {
                    schema
                        .fields
                        .iter()
                        .position(|f| f == p)
                        // we can remove later if we want silent filter (if not found)
                        // which can be useful once we have multiple scanners (multi-table queries)
                        .expect(&format!(
                            "'{p}' field not found in table '{}'",
                            schema.table
                        ))
                })
                .collect(),
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

        Some(
            row.into_iter()
                .enumerate()
                .filter(|(i, _)| self.idxs.contains(i))
                .map(|(_, s)| s)
                .collect(),
        )
    }
}

fn main() {
    let query = fs::read_to_string("query.json").unwrap();
    let json: Value = serde_json::from_str(&query).unwrap();
    let query = Query::from(json);

    let scan = query
        .scan
        .expect("there should be at least one table in the scan list");
    // TODO: multiple scanners
    let mut scanner = FileScan::new(&scan[0]);

    let results: Vec<_> = match query.projection {
        Some(projection) => {
            let schema = Schema::new(scanner.table());
            let projector = Projector::new(projection, &mut scanner, schema);
            projector.into_iter().collect()
        }
        None => scanner.into_iter().collect(),
    };

    println!("results:");
    println!("{results:?}");
}
