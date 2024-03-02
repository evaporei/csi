use std::fs::read_to_string;

use db::query::Query;
use db::source::{FileScan, Projector, Selector, Row, Schema, Metadata};
use db::index::IndexBuilder;

// const QUERY: &str = "queries/simple.json";
// const QUERY: &str = "queries/multi-table.json";
const QUERY: &str = "queries/join.json";

fn main() {
    let query = read_to_string(QUERY).unwrap();
    let json: serde_json::Value = serde_json::from_str(&query).unwrap();
    let query = Query::from(json);

    let scan = query
        .scan
        .expect("there should be at least one table in the scan list");
    let scanners: Vec<FileScan> = scan.iter().map(|table| FileScan::new(&table)).collect();

    // TODO: ewww, make this pretty plsss
    // this means we're doing table.field in the query, aka JOINs!!!
    let should_join = query
        .selection
        .as_ref()
        .map(|some| some.iter().any(|f| f.contains(".")))
        .unwrap_or(false)
        || query
            .projection
            .as_ref()
            .map(|some| some.iter().any(|f| f.contains(".")))
            .unwrap_or(false);
    if should_join {
        assert_eq!(scan.len(), 2, "for now just JOINs w/ two tables");
        println!("woo join");
    } else {
        for mut scanner in scanners {
            let schema = Schema::new(scanner.table());

            let results: Vec<Row> = match (query.selection.clone(), query.projection.clone()) {
                (Some(selection), Some(projection)) => {
                    let mut selector = Selector::new(selection, &mut scanner, &schema)
                        .into_iter()
                        .flatten();
                    let projector = Projector::new(projection, &mut selector, &schema);
                    projector.into_iter().collect()
                }
                (Some(selection), None) => {
                    let selector = Selector::new(selection, &mut scanner, &schema);
                    selector.into_iter().flatten().collect()
                }
                (None, Some(projection)) => {
                    let projector = Projector::new(projection, &mut scanner, &schema);
                    projector.into_iter().collect()
                }
                (None, None) => scanner.into_iter().collect(),
            };

            println!("results:");
            println!("{results:?}");
        }
    }

    // index example/tests
    let schema = Schema::new("movies");
    let mut scanner = FileScan::new("movies");
    // meh, this is a vec... bleeping FromIterator
    let index = IndexBuilder::new("movieId", &mut scanner, &schema)
        .into_iter()
        // uhhh I don't know about this...
        // maybe I don't need an iterator for the
        // IndexBuilder? :thinking:
        .collect::<Vec<_>>()
        .pop()
        .unwrap();
    println!("bin search:");
    println!("{:?}", index.search("5000"));
}
