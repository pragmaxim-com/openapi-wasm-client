### Rust playground

[rust-rocksdb](https://github.com/rust-rocksdb/rust-rocksdb) leverages rustc to prevent us from misusing rocksdb,
however sometimes it is very tricky to achieve what we need, for instance complex column family management.

Rust lifetimes are often hacked in a way that we can spent half a day with trying to compile code base after refactoring.