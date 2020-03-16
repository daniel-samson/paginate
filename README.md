A framework agnostic pagination crate, that is especially suited for databases, collections and web pages.

### Example

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

