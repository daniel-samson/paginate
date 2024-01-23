A framework agnostic pagination crate, that is especially suited for databases, slices and collections. Paginate calculates the range of page indexes, making it ideal for accessing slices, chunking data and querying databases.


## Project Status

[![crates.io](https://img.shields.io/crates/v/paginate.svg)](https://crates.io/crates/paginate)
[![CI](https://github.com/daniel-samson/paginate/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/daniel-samson/paginate/actions/workflows/ci.yml)
[![CodeFactor](https://www.codefactor.io/repository/github/daniel-samson/paginate/badge)](https://www.codefactor.io/repository/github/daniel-samson/paginate)
[![codecov](https://codecov.io/gh/daniel-samson/paginate/branch/main/graph/badge.svg)](https://codecov.io/gh/daniel-samson/paginate)
[![book](https://img.shields.io/badge/Book-Latest-blue)](https://daniel-samson.github.io/paginate-docs/)
[![stars - paginate](https://img.shields.io/github/stars/daniel-samson/paginate?style=social)](https://github.com/daniel-samson/paginate)
[![forks - paginate](https://img.shields.io/github/forks/daniel-samson/paginate?style=social)](https://github.com/daniel-samson/paginate)

## Installation
Add the following line to your Cargo.toml file:

```toml
paginate = "^1"
```

## Examples


To iterate over each page:

```rust
use paginate::Pages;

fn main() {
    let total_items = 100;
    let items_per_page = 5;
    let pages = Pages::new(total_items, items_per_page);
    println!("total pages: {}", pages.page_count());
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
    println!("total pages: {}", pages.page_count());
    println!("offset: {}, total: {}, start: {}, end: {}", page.offset, page.length, page.start, page.end);
}
```

## Getting help

- [API Documentation](https://docs.rs/paginate/latest/paginate/)
- [Book](https://daniel-samson.github.io/paginate-docs/)


## Contribute

- Star this project. It really helps!
- [Ask a question](https://github.com/daniel-samson/paginate/issues/new?assignees=&labels=question&template=question.md&title=Question%3A+)
- [Report a bug](https://github.com/daniel-samson/paginate/issues/new?assignees=&labels=bug&template=bug_report.md&title=Bug+Report%3A+)
- [Request documentation](https://github.com/daniel-samson/paginate/issues/new?assignees=&labels=documentation&template=documentation.md&title=Needs+Documentation%3A+)
- [Request a new feature](https://github.com/daniel-samson/paginate/issues/new?assignees=&labels=enhancement&template=feature_request.md&title=)

