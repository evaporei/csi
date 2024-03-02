use std::fs::read_to_string;

use db::index::IndexBuilder;
use db::query::Query;
use db::source::{FileScan, Metadata, HashJoin, Projector, Row, Schema, Selector};

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
    let mut scanners: Vec<FileScan> = scan.iter().map(|table| FileScan::new(&table)).collect();

    if let Some(join) = query.join {
        assert_eq!(scan.len(), 2, "for now just JOINs w/ two tables");
        let mut inner = scanners.remove(1);
        let mut outer = scanners.remove(0);
        let outer_schema = Schema::new(outer.table());
        let inner_schema = Schema::new(inner.table());
        // let join = NestedJoin::new(&mut outer, &mut inner, outer_schema, inner_schema, join);
        let join = HashJoin::new(&mut outer, &mut inner, outer_schema, inner_schema, join);
        let results: Vec<_> = join.into_iter().flatten().collect();
        println!("results:");
        println!("{results:?}");
    } else {
        // single or multi-table queries (no JOINs)
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
