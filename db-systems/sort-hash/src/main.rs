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

use std::collections::BTreeSet;

// TODO: performance (it could be an int)
type Genre = String;

#[derive(Debug)]
struct Movie {
    id: usize,
    title: String,
    genres: BTreeSet<Genre>,
}

impl From<String> for Movie {
    fn from(s: String) -> Self {
        let mut values = s.split(',');
        let id = values.next().unwrap().parse().unwrap();
        let title = values.next().unwrap().into();
        let mut genres: BTreeSet<_> = values
            .next()
            .unwrap()
            .split('|')
            .map(|a| a.to_owned())
            .collect();

        if genres.contains("(no genres listed)") {
            genres.remove("(no genres listed)");
        }

        Self { id, title, genres }
    }
}

#[derive(Debug)]
struct MovieView {
    id: Option<usize>,
    title: Option<String>,
    genres: Option<BTreeSet<Genre>>,
}

impl From<Movie> for MovieView {
    fn from(m: Movie) -> Self {
        Self {
            id: Some(m.id),
            title: Some(m.title),
            genres: Some(m.genres),
        }
    }
}

use std::fmt;

impl fmt::Display for MovieView {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{")?;
        if let Some(id) = self.id {
            write!(f, "{id}, ")?;
        }
        if let Some(title) = &self.title {
            write!(f, "{title}, ")?;
        }
        if let Some(genres) = &self.genres {
            write!(f, "{genres:?}, ")?;
        }
        write!(f, "}}")
    }
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

fn main() {
    let query = fs::read_to_string("query.json").unwrap();
    let json: Value = serde_json::from_str(&query).unwrap();
    let query = Query::from(json);
    let mut results = vec![];

    if let Some(tables) = query.scan {
        for table in tables {
            let mut rows = vec![];
            let lines = read_lines(format!("./ml-20m/{table}.csv")).unwrap().skip(1);

            for line in lines {
                let line = line.unwrap();
                // TODO: we don't know table structure at compile-time
                let movie = Movie::from(line);
                rows.push(MovieView::from(movie));
            }
            results.push(rows);
        }
    }

    if let Some(conditions) = query.selection {
        if conditions[1] == "EQUALS" {
            if conditions[0] == "id" {
                results[0].retain(|m| m.id.unwrap() == conditions[2].parse::<usize>().unwrap());
            }
            if conditions[0] == "title" {
                results[0].retain(|m| m.title.as_ref().unwrap() == &conditions[2]);
            }
            if conditions[0] == "genres" {
                results[0].retain(|m| m.genres.as_ref().unwrap().contains(&conditions[2]));
            }
        }
    }

    if let Some(attributes) = query.projection {
        // this is ridiculous...
        if !attributes.contains(&"id".to_owned()) {
            for table in results.iter_mut() {
                for movie in table {
                    movie.id = None;
                }
            }
        }
        // this is ridiculous...
        if !attributes.contains(&"title".to_owned()) {
            for table in results.iter_mut() {
                for movie in table {
                    movie.title = None;
                }
            }
        }
        // this is ridiculous...
        if !attributes.contains(&"genres".to_owned()) {
            for table in results.iter_mut() {
                for movie in table {
                    movie.genres = None;
                }
            }
        }
    }

    println!("results:");
    println!("{results:?}");
    // println!("{results}"); // oh fuck Vec doesn't impl fmt::Display
}
