A framework agnostic pagination crate, that is especially suited for databases, slices and collections. Paginate calculates the range of pages indexes, making it ideal for accessing slices, chunking data and querying databases.


## Project Status

[![crates.io](https://img.shields.io/crates/v/paginate.svg)](https://crates.io/crates/paginate)
[![Rust](https://github.com/daniel-samson/paginate/actions/workflows/rust.yml/badge.svg?branch=master)](https://github.com/daniel-samson/paginate/actions/workflows/rust.yml)
[![Docs](https://docs.rs/paginate/badge.svg?version=1.1.0)](https://docs.rs/paginate/)
[![codecov](https://codecov.io/gh/daniel-samson/paginate/branch/master/graph/badge.svg)](https://codecov.io/gh/daniel-samson/paginate)
[![book](https://img.shields.io/badge/Book-v1.1.0-blue)](https://daniel-samson.github.io/paginate-docs/)


## Examples


To iterate over each page:

```rust
use paginate::Pages;

fn main() {
    let total_items = 100;
    let items_per_page = 5;
    let pages = Pages::new(total_items, items_per_page);
    for page in pages.into_iter() {
        println!("offset: {}, total: {}, start: {}, end: {}", page.offset, page.length, page.start, page.end);
    }
}
```

To get the pagination of a specific offset:
```rust
use paginate::Pages;

fn main() {
    let total_items = 35;
    let items_per_page = 5;
    let pages = Pages::new(total_items, items_per_page);
    let page = pages.with_offset(3);
    println!("offset: {}, total: {}, start: {}, end: {}", page.offset, page.length, page.start, page.end);
}
```

## Getting help

- [API Documentation](https://docs.rs/paginate/latest/paginate/)
- [Book](https://daniel-samson.github.io/paginate-docs/)


## Contribute

- [Ask a question](https://github.com/daniel-samson/paginate/issues/new?assignees=&labels=question&template=question.md&title=Question%3A+)
- [Report a bug](https://github.com/daniel-samson/paginate/issues/new?assignees=&labels=bug&template=bug_report.md&title=Bug+Report%3A+)
- [Request documentation](https://github.com/daniel-samson/paginate/issues/new?assignees=&labels=documentation&template=documentation.md&title=Needs+Documentation%3A+)
- [Request a new feature](https://github.com/daniel-samson/paginate/issues/new?assignees=&labels=&template=feature_request.md&title=)

