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
    lines: Skip<io::Lines<io::BufReader<File>>>,
}

impl FileScan {
    fn new(table: &str) -> Self {
        Self {
            lines: read_lines(format!("./ml-20m/{table}.csv")).unwrap().skip(1),
        }
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

fn main() {
    let query = fs::read_to_string("query.json").unwrap();
    let json: Value = serde_json::from_str(&query).unwrap();
    let _query = Query::from(json);

    let file = FileScan::new("movies");

    let mut results = vec![];

    for row in file {
        results.push(row);
    }

    println!("results:");
    println!("{results:?}");
}
