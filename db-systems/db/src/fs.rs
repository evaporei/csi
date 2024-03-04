use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, Read, Seek, SeekFrom, Write};
use std::path::Path;

use crate::source::Row;

pub fn buf_reader<P>(filename: P) -> io::Result<io::BufReader<File>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file))
}

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    buf_reader(filename).map(|b| b.lines())
}

pub struct HeapFile {
    ptr_lower: usize,
    ptr_upper: usize,
    // TODO: convert those two into one structure
    writer: io::BufWriter<File>,
    reader: io::BufReader<File>,
}

// the code here is messy
// I'm still experimenting with the file API in a
// procedural manner before I abstract things
impl HeapFile {
    pub fn create(table: &str) -> Self {
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            // maybe change folder if cfg(test)?
            .open(format!("./data/{table}"))
            .unwrap();

        // 8 (64 bits) * 2
        let ptr_lower: usize = 16;
        // end of block
        let ptr_upper: usize = 8192;
        file.write_all(&ptr_lower.to_be_bytes()).unwrap();
        file.write_all(&ptr_upper.to_be_bytes()).unwrap();

        // fill whole block
        file.write_all(&[0; 8192 - 16]).unwrap();

        Self {
            ptr_lower,
            ptr_upper,
            writer: io::BufWriter::new(file.try_clone().unwrap()),
            reader: io::BufReader::new(file),
        }
    }

    pub fn open(table: &str) -> Self {
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(format!("./data/{table}"))
            .unwrap();

        let mut ptr_lower = [0; 8];
        file.read_exact(&mut ptr_lower).unwrap();

        let mut ptr_upper = [0; 8];
        file.read_exact(&mut ptr_upper).unwrap();

        Self {
            ptr_lower: usize::from_be_bytes(ptr_lower),
            ptr_upper: usize::from_be_bytes(ptr_upper),
            writer: io::BufWriter::new(file.try_clone().unwrap()),
            reader: io::BufReader::new(file),
        }
    }

    pub fn insert(&mut self, row: Row) {
        // TODO (important): check extra space size (if we're able to write)
        // otherwise: new block (oh boy that will be a lot of work)
        let mut buffer = vec![];
        for column in row {
            buffer.write_all(&column.len().to_be_bytes()).unwrap();
            buffer.write_all(&column.as_bytes()).unwrap();
        }
        let buffer_len = buffer.len(); // fixme
        let new_upper = self.ptr_upper - buffer_len - 8;
        // maybe use SeekFrom::End and set ptr_upper to the result of .seek()
        // though in the future multiple pages might complicate things
        self.writer
            .seek(SeekFrom::Start((new_upper) as u64))
            .unwrap();
        self.writer.write_all(&buffer_len.to_be_bytes()).unwrap();
        self.writer.write_all(&buffer).unwrap();
        self.update_ptrs(new_upper);
        // we'll see if we should keep this
        self.writer.flush().unwrap();
    }

    // header & line ptrs shenanigans
    fn update_ptrs(&mut self, new_upper: usize) {
        // let's write the header
        self.writer.seek(SeekFrom::Start(0)).unwrap();
        // new line ptr
        self.writer
            .write_all(&(self.ptr_lower + 8).to_be_bytes())
            .unwrap();
        self.writer.write_all(&new_upper.to_be_bytes()).unwrap();
        self.writer
            .seek(SeekFrom::Start(self.ptr_lower as u64))
            .unwrap();
        // update local
        self.ptr_upper = new_upper;
        self.ptr_lower += 8;
        // write new line ptr
        self.writer.write_all(&new_upper.to_be_bytes()).unwrap();
    }

    // starts at 0
    // needs to be mut because of the underlying file buffers (maybe FIXME?)
    pub fn get(&mut self, n: usize) -> Option<Row> {
        let offset = 16 + 8 * n;
        self.reader.seek(SeekFrom::Start(offset as u64)).unwrap();
        let mut line_ptr = [0; 8];
        self.reader.read_exact(&mut line_ptr).unwrap();
        let line_ptr = usize::from_be_bytes(line_ptr);
        // we wrote all zeroes previously
        if line_ptr == 0 {
            return None;
        }
        self.reader.seek(SeekFrom::Start(line_ptr as u64)).unwrap();
        // we can read up until this
        let mut tuple_size = [0; 8];
        self.reader.read_exact(&mut tuple_size).unwrap();
        let tuple_size = usize::from_be_bytes(tuple_size);
        let mut raw_row = vec![0; tuple_size];
        self.reader.read_exact(&mut raw_row).unwrap();
        let mut row = vec![];
        let mut curr = 0;
        while curr < tuple_size {
            let field_len = usize::from_be_bytes(raw_row[curr..curr + 8].try_into().unwrap());
            // TODO: maybe there's a way to leverage ptr & unsafe (from_raw_parts)
            // to avoid the .to_vec() allocation
            let field =
                String::from_utf8(raw_row[curr + 8..curr + 8 + field_len].to_vec()).unwrap();
            row.push(field);
            curr += 8 + field_len;
        }
        Some(row)
    }
}

pub struct HeapIterator {
    n: usize,
    heap: HeapFile,
}

impl IntoIterator for HeapFile {
    type Item = Row;
    type IntoIter = HeapIterator;

    fn into_iter(self) -> Self::IntoIter {
        HeapIterator { n: 0, heap: self }
    }
}

impl Iterator for HeapIterator {
    type Item = Row;
    fn next(&mut self) -> Option<Self::Item> {
        let row = self.heap.get(self.n)?;
        self.n += 1;
        Some(row)
    }
}

#[test]
fn test_heap_file() {
    let _heap = HeapFile::create("test_movies");

    let expected: [u8; 16] = [
        // 16
        0, 0, 0, 0, 0, 0, 0, 16, // 8192
        0, 0, 0, 0, 0, 0, 32, 0,
    ];
    let mut header = [0; 16];
    let mut f = File::open("./data/test_movies").unwrap();
    f.read_exact(&mut header).unwrap();
    assert_eq!(header, expected);

    let mut heap = HeapFile::open("test_movies");

    assert_eq!(heap.ptr_lower, 16);
    assert_eq!(heap.ptr_upper, 8192);

    let movie = vec![
        // -------- length ------- 1
        // 00 00 00 00 00 00 00 01 31
        "1".into(),
        // ------- length -------- T  o  y  \s S  t  o  r  y
        // 00 00 00 00 00 00 00 09 54 6f 79 20 53 74 6f 72 79
        "Toy Story".into(),
        // ------- length -------- A  n  i  m  a  t  i  o  n
        // 00 00 00 00 00 00 00 09 41 6e 69 6d 61 74 69 6f 6e
        "Animation".into(),
    ];
    heap.insert(movie.clone());

    let expected = [
        // upper length
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x2b, // length
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, // "1"
        0x31, // length
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x09, // "Toy Story"
        0x54, 0x6f, 0x79, 0x20, 0x53, 0x74, 0x6f, 0x72, 0x79, // length
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x09, // "Animation"
        0x41, 0x6e, 0x69, 0x6d, 0x61, 0x74, 0x69, 0x6f, 0x6e,
    ];

    let new_upper = 8192 - expected.len();
    assert_eq!(heap.ptr_upper, new_upper);
    assert_eq!(heap.ptr_lower, 24);

    f.seek(SeekFrom::Start((8192 - expected.len()) as u64))
        .unwrap();
    // f.seek(SeekFrom::End(-(expected.len() as i64))).unwrap();
    let mut found = [0; 51];
    f.read_exact(&mut found).unwrap();
    assert_eq!(found, expected);

    let expected: [u8; 16] = [
        // 16
        0, 0, 0, 0, 0, 0, 0, 24, // 8192
        0, 0, 0, 0, 0, 0, 0x1f, 0xcd,
    ];
    let mut header = [0; 16];
    f.seek(SeekFrom::Start(0)).unwrap();
    f.read_exact(&mut header).unwrap();
    assert_eq!(header, expected);

    assert_eq!(heap.get(0).unwrap(), movie);
}

#[test]
fn test_heap_file_iterator() {
    let mut heap = HeapFile::create("test_it");

    let movies = vec![
        vec![
            "1".into(),
            "Toy Story (1995)".into(),
            "Adventure|Animation|Children|Comedy|Fantasy".into(),
        ],
        vec![
            "2".into(),
            "Jumanji (1995)".into(),
            "Adventure|Children|Fantasy".into(),
        ],
        vec![
            "3".into(),
            "Grumpier Old Men (1995)".into(),
            "Comedy|Romance".into(),
        ],
        vec![
            "4".into(),
            "Waiting to Exhale (1995)".into(),
            "Comedy|Drama|Romance".into(),
        ],
        vec![
            "5".into(),
            "Father of the Bride Part II (1995)".into(),
            "Comedy".into(),
        ],
    ];

    for movie in &movies {
        heap.insert(movie.clone());
    }

    assert_eq!(heap.into_iter().collect::<Vec<_>>(), movies);
}
