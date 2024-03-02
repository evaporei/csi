// this is a copy/adaptation of one of the iterations of the
// BTreeMap from the Rust standard library
// commit in the compiler: b6edc59413f79016a1063c2ec6bc05516bc99cb6
pub mod btree;
// still doesn't abstract all fs operations
// they are scattered across the code base
// I'll fix that later
pub mod fs;
pub mod index;
pub mod query;
pub mod source;
