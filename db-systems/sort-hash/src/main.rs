use std::fs::File;
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

fn main() {
    let lines = read_lines("./ml-20m/movies.csv").unwrap().skip(1);
    let mut movies = vec![];

    for line in lines {
        let line = line.unwrap();
        println!("{line}");
        let movie = Movie::from(line);
        println!("{movie:?}");
        movies.push(movie);
    }
}
