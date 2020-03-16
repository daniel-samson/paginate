A framework agnostic pagination crate, that is especially suited for databases, collections and web pages.

![Rust](https://github.com/daniel-samson/paginate/workflows/Rust/badge.svg?branch=master)
![Docs](https://docs.rs/paginate/badge.svg?version=1.0.0)

### Example


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

