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
    free_space: usize,
    // TODO: convert those two into one structure
    writer: io::BufWriter<File>,
    reader: io::BufReader<File>,
}

// the code here is messy
// I'm still experimenting with the file API in a
// procedural manner before I abstract things
impl HeapFile {
    pub fn create(table: &str) -> Result<Self, io::Error> {
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            // maybe change folder if cfg(test)?
            .open(format!("./data/{table}"))?;

        // 8 (64 bits) * 2
        let ptr_lower: usize = 16;
        // end of block
        let ptr_upper: usize = 8192;
        file.write_all(&ptr_lower.to_be_bytes())?;
        file.write_all(&ptr_upper.to_be_bytes())?;

        // fill whole block
        file.write_all(&[0; 8192 - 16])?;

        let used = 8 + 8;

        Ok(Self {
            ptr_lower,
            ptr_upper,
            free_space: ptr_upper - used,
            writer: io::BufWriter::new(file.try_clone()?),
            reader: io::BufReader::new(file),
        })
    }

    pub fn open(table: &str) -> Result<Self, io::Error> {
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(format!("./data/{table}"))?;

        let mut ptr_lower = [0; 8];
        file.read_exact(&mut ptr_lower)?;

        let mut ptr_upper = [0; 8];
        file.read_exact(&mut ptr_upper)?;

        let ptr_lower = usize::from_be_bytes(ptr_lower);
        let ptr_upper = usize::from_be_bytes(ptr_upper);

        let ptrs = (ptr_lower - 16) / 8;
        let tuples = 8192 - ptr_upper;
        let used = 8 + 8 + ptrs + tuples;

        Ok(Self {
            ptr_lower,
            ptr_upper,
            free_space: 8192 - used,
            writer: io::BufWriter::new(file.try_clone().unwrap()),
            reader: io::BufReader::new(file),
        })
    }

    pub fn insert(&mut self, row: Row) -> Result<(), io::Error> {
        // TODO (important): check extra space size (if we're able to write)
        // otherwise: new block (oh boy that will be a lot of work)
        let mut buffer = vec![];
        for column in row {
            buffer.write_all(&column.len().to_be_bytes())?;
            buffer.write_all(&column.as_bytes())?;
        }
        let buffer_len = buffer.len(); // fixme
        let new_upper = self.ptr_upper - buffer_len - 8;

        if buffer_len + 8 > self.free_space {
            panic!("no more space in heap file");
        }

        // maybe use SeekFrom::End and set ptr_upper to the result of .seek()
        // though in the future multiple pages might complicate things
        self.writer.seek(SeekFrom::Start((new_upper) as u64))?;
        self.writer.write_all(&buffer_len.to_be_bytes())?;
        self.writer.write_all(&buffer)?;
        self.update_ptrs(new_upper)?;
        self.free_space -= 8 + buffer_len;
        // we'll see if we should keep this
        self.writer.flush()?;
        Ok(())
    }

    // header & line ptrs shenanigans
    fn update_ptrs(&mut self, new_upper: usize) -> Result<(), io::Error> {
        // let's write the header
        self.writer.seek(SeekFrom::Start(0))?;
        // new line ptr
        self.writer.write_all(&(self.ptr_lower + 8).to_be_bytes())?;
        self.writer.write_all(&new_upper.to_be_bytes())?;
        self.writer.seek(SeekFrom::Start(self.ptr_lower as u64))?;
        // update local
        self.ptr_upper = new_upper;
        self.ptr_lower += 8;
        // write new line ptr
        self.writer.write_all(&new_upper.to_be_bytes())?;
        Ok(())
    }

    // starts at 0
    // needs to be mut because of the underlying file buffers (maybe FIXME?)
    pub fn get(&mut self, n: usize) -> Result<Option<Row>, io::Error> {
        let offset = 16 + 8 * n;
        self.reader.seek(SeekFrom::Start(offset as u64))?;
        let mut line_ptr = [0; 8];
        self.reader.read_exact(&mut line_ptr)?;
        let line_ptr = usize::from_be_bytes(line_ptr);
        // we wrote all zeroes previously
        if line_ptr == 0 {
            return Ok(None);
        }
        self.reader.seek(SeekFrom::Start(line_ptr as u64))?;
        // we can read up until this
        let mut tuple_size = [0; 8];
        self.reader.read_exact(&mut tuple_size)?;
        let tuple_size = usize::from_be_bytes(tuple_size);
        let mut raw_row = vec![0; tuple_size];
        self.reader.read_exact(&mut raw_row)?;
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
        Ok(Some(row))
    }
}

pub struct HeapIterator {
    n: usize,
    heap: HeapFile,
}

impl IntoIterator for HeapFile {
    type Item = Result<Row, io::Error>;
    type IntoIter = HeapIterator;

    fn into_iter(self) -> Self::IntoIter {
        HeapIterator { n: 0, heap: self }
    }
}

impl Iterator for HeapIterator {
    type Item = Result<Row, io::Error>;
    fn next(&mut self) -> Option<Self::Item> {
        let row = self.heap.get(self.n).transpose()?;
        self.n += 1;
        Some(row)
    }
}

#[test]
fn test_heap_file() {
    let heap = HeapFile::create("test_movies").unwrap();

    let expected: [u8; 16] = [
        // 16
        0, 0, 0, 0, 0, 0, 0, 16, // 8192
        0, 0, 0, 0, 0, 0, 32, 0,
    ];
    let mut header = [0; 16];
    let mut f = File::open("./data/test_movies").unwrap();
    f.read_exact(&mut header).unwrap();
    assert_eq!(header, expected);
    assert_eq!(heap.free_space, 8176);

    let mut heap = HeapFile::open("test_movies").unwrap();

    assert_eq!(heap.ptr_lower, 16);
    assert_eq!(heap.ptr_upper, 8192);
    // remains the same with ::open()
    assert_eq!(heap.free_space, 8176);

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
    heap.insert(movie.clone()).unwrap();

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
    assert_eq!(heap.free_space, 8192 - 16 - expected.len());

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

    assert_eq!(heap.get(0).unwrap(), Some(movie));
}

#[test]
fn test_heap_file_iterator() {
    let mut heap = HeapFile::create("test_it").unwrap();

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
        heap.insert(movie.clone()).unwrap();
    }

    assert_eq!(
        heap.into_iter().map(Result::unwrap).collect::<Vec<_>>(),
        movies
    );
}

#[test]
#[should_panic(expected = "no more space in heap file")]
fn test_heap_full() {
    let movie = vec![
        "1".into(),
        "Toy Story (1995)".into(),
        "Adventure|Animation|Children|Comedy|Fantasy".into(),
    ];

    // after 89 of the movie above, we have no more extra space to
    // fit the same movie again
    let movies = std::iter::repeat(movie).take(89);

    let mut heap = HeapFile::create("test_full").unwrap();

    for movie in movies {
        heap.insert(movie).unwrap();
    }
}
