//! A framework agnostic pagination crate, that is especially suited for databases, collections and web pages.
//!
//! ### Example
//!
//! To iterate over each page:
//! ```rust
//! use paginate::Pages;
//!
//! fn print_pages() {
//!     let total_items = 100usize;
//!      let items_per_page = 5usize;
//!      let pages = Pages::new(total_items, items_per_page);
//!      for page in pages.into_iter() {
//!          println!("offset: {}, total: {}, start: {}, end: {}", page.offset, page.length, page.start, page.end);
//!      }
//! }
//! ```
//!
//! To get the pagination of a specific offset:
//! ```rust
//! use paginate::Pages;
//!
//! fn print_test() {
//!     let total_items = 35;
//!     let items_per_page = 5;
//!     let pages = Pages::new(total_items, items_per_page);
//!     let page = pages.with_offset(3);
//!     println!("offset: {}, total: {}, start: {}, end: {}", page.offset, page.length, page.start, page.end);
//! }
//! ```

use std::cmp::{max, min};

///
/// Defines the pages to facilitate pagination.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Pages {
    /// Current page offset.
    offset: usize,
    /// Total number of items.
    length: usize,
    /// Total number of pages.
    limit: usize,
}

impl Pages {
    /// Defines a new pagination.
    ///
    /// ### Arguments
    ///
    /// * `length` - the total number of items to paginate.
    /// * `limit` - the maximum number of items that can exist on a single page.
    pub fn new(length: usize, limit: usize) -> Pages {
        Pages {
            offset: 0,
            length,
            limit,
        }
    }

    /// Gets a specific page.
    ///
    /// ### Arguments
    ///
    /// * `offset` - the offset of the page you want to retrieve.
    pub fn with_offset(&self, offset: usize) -> Page {
        let mut page = Page::default();
        page.offset = offset;
        page.start = min(page.offset * self.limit, self.length);
        page.end = min(page.start + self.limit, self.length);
        page.length = max(page.end - page.start, 0);
        if page.length == 0 {
            page.start = 0;
            page.end = 0;
        };
        if page.length > 0 {
            page.end -= 1;
        };
        page
    }

    /// Gets the current page offset.
    pub fn offset(&self) -> usize {
        self.offset
    }

    /// Gets the total number of items.
    pub fn length(&self) -> usize {
        self.length
    }

    /// Gets the maximum number of items per page.
    pub fn limit(&self) -> usize {
        self.limit
    }

    /// Gets the total number of pages.
    pub fn page_count(&self) -> usize {
        (self.length + self.limit - 1) / self.limit
    }
}

impl Iterator for Pages {
    type Item = Page;
    fn next(&mut self) -> Option<Self::Item> {
        let page: Page = self.with_offset(self.offset);
        self.offset += 1;
        if page.is_empty() {
            None
        } else {
            Some(page)
        }
    }
}

impl IntoIterator for &Pages {
    type Item = Page;
    type IntoIter = Pages;

    fn into_iter(self) -> Pages {
        self.clone()
    }
}

/// Defines the properties of a page.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Page {
    /// The current page offset.
    pub offset: usize,
    /// The total number of items that exist on this page offset.
    pub length: usize,
    /// The index of the first item on this page.
    pub start: usize,
    /// The index of the last item on this page.
    pub end: usize,
}

impl Page {
    /// When a page does not contain any items it is considered to be empty.
    pub fn is_empty(&self) -> bool {
        self.length == 0
    }
}

impl Default for Page {
    /// Creates an empty page with no items.
    fn default() -> Self {
        Page {
            offset: 0usize,
            length: 0usize,
            start: 0usize,
            end: 0usize,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Page, Pages};

    #[test]
    fn default_page() {
        let page = Page::default();
        assert_eq!(page, Page { offset: 0, length: 0, start: 0, end: 0 });
    }

    #[test]
    fn empty_page() {
        let total_items = 0usize;
        let items_per_page = 5usize;
        let pages = Pages::new(total_items, items_per_page);
        assert_eq!(pages.with_offset(0), Page { offset: 0, length: 0, start: 0, end: 0 });
        assert_eq!(pages.with_offset(1), Page { offset: 1, length: 0, start: 0, end: 0 });
    }

    #[test]
    fn limitless_page() {
        let total_items = 5usize;
        let items_per_page = 0usize;
        let pages = Pages::new(total_items, items_per_page);
        assert_eq!(pages.with_offset(0), Page { offset: 0, length: 0, start: 0, end: 0 });
        assert_eq!(pages.with_offset(1), Page { offset: 1, length: 0, start: 0, end: 0 });
    }

    #[test]
    fn single_page() {
        let total_items = 5usize;
        let items_per_page = 5usize;
        let pages = Pages::new(total_items, items_per_page);
        assert_eq!(pages.with_offset(0), Page { offset: 0, length: 5, start: 0, end: 4 });
        assert_eq!(pages.with_offset(1), Page { offset: 1, length: 0, start: 0, end: 0 });
        assert_eq!(pages.with_offset(2), Page { offset: 2, length: 0, start: 0, end: 0 });
    }

    #[test]
    fn single_item() {
        let total_items = 1usize;
        let items_per_page = 5usize;
        let pages = Pages::new(total_items, items_per_page);
        assert_eq!(pages.with_offset(0), Page { offset: 0, length: 1, start: 0, end: 0 });
        assert_eq!(pages.with_offset(1), Page { offset: 1, length: 0, start: 0, end: 0 });
    }

    #[test]
    fn odd_items() {
        let total_items = 5usize;
        let items_per_page = 2usize;
        let pages = Pages::new(total_items, items_per_page);
        assert_eq!(pages.with_offset(0), Page { offset: 0, length: 2, start: 0, end: 1 });
        assert_eq!(pages.with_offset(1), Page { offset: 1, length: 2, start: 2, end: 3 });
        assert_eq!(pages.with_offset(2), Page { offset: 2, length: 1, start: 4, end: 4 });
        assert_eq!(pages.with_offset(3), Page { offset: 3, length: 0, start: 0, end: 0 });
    }

    #[test]
    fn even_items() {
        let total_items = 6usize;
        let items_per_page = 2usize;
        let pages = Pages::new(total_items, items_per_page);
        assert_eq!(pages.with_offset(0), Page { offset: 0, length: 2, start: 0, end: 1 });
        assert_eq!(pages.with_offset(1), Page { offset: 1, length: 2, start: 2, end: 3 });
        assert_eq!(pages.with_offset(2), Page { offset: 2, length: 2, start: 4, end: 5 });
        assert_eq!(pages.with_offset(3), Page { offset: 3, length: 0, start: 0, end: 0 });
    }

    #[test]
    fn odd_sizes() {
        let total_items = 5usize;
        let items_per_page = 3usize;
        let pages = Pages::new(total_items, items_per_page);
        assert_eq!(pages.with_offset(0), Page { offset: 0, length: 3, start: 0, end: 2 });
        assert_eq!(pages.with_offset(1), Page { offset: 1, length: 2, start: 3, end: 4 });
        assert_eq!(pages.with_offset(2), Page { offset: 2, length: 0, start: 0, end: 0 });
    }

    #[test]
    fn iterator() {
        let total_items = 1usize;
        let items_per_page = 1usize;
        let pages = Pages::new(total_items, items_per_page);
        for p in pages.into_iter() {
            assert_eq!(p, Page { offset: 0, length: 1, start: 0, end: 0 });
        }
    }

    #[test]
    fn iterator_ref() {
        let total_items = 1usize;
        let items_per_page = 1usize;
        let pages = Pages::new(total_items, items_per_page);
        for p in &pages {
            assert_eq!(p, Page { offset: 0, length: 1, start: 0, end: 0 });
        }
    }

    #[test]
    fn is_empty() {
        let empty_page = Page::default();
        assert!(empty_page.is_empty());

        let filled_page = Page {
            length: 1,
            ..Page::default()
        };
        assert!(!filled_page.is_empty());
    }

    #[test]
    fn offset() {
        let pages = Pages::new(100, 5);
        assert_eq!(0, pages.offset());
    }

    #[test]
    fn length() {
        let pages = Pages::new(100, 5);
        assert_eq!(100, pages.length());
    }

    #[test]
    fn limit() {
        let pages = Pages::new(100, 5);
        assert_eq!(5, pages.limit());
    }

    #[test]
    fn page_count() {
        let pages = Pages::new(100, 5);
        assert_eq!(20, pages.page_count());

        let pages = Pages::new(101, 5);
        assert_eq!(21, pages.page_count());

        let pages = Pages::new(99, 5);
        assert_eq!(20, pages.page_count());
    }
}
