//! Pagination types for API responses.

use serde::{Deserialize, Serialize};

/// Pagination information for list responses.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageInfo {
    /// Current page number (1-indexed)
    pub page: u32,
    /// Number of items per page
    pub page_size: u32,
    /// Total number of items across all pages
    pub total_items: u64,
    /// Total number of pages
    pub total_pages: u32,
}

impl PageInfo {
    /// Create new pagination info.
    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    pub fn new(page: u32, page_size: u32, total_items: u64) -> Self {
        let total_pages = if total_items == 0 || page_size == 0 {
            0
        } else {
            total_items.div_ceil(u64::from(page_size)) as u32
        };

        Self {
            page,
            page_size,
            total_items,
            total_pages,
        }
    }

    /// Check if there is a next page.
    #[must_use]
    pub const fn has_next(&self) -> bool {
        self.page < self.total_pages
    }

    /// Check if there is a previous page.
    #[must_use]
    pub const fn has_previous(&self) -> bool {
        self.page > 1
    }
}

/// Paginated response wrapper.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Paginated<T> {
    /// The data items for this page
    pub data: Vec<T>,
    /// Pagination information
    pub pagination: PageInfo,
}

impl<T> Paginated<T> {
    /// Create a new paginated response.
    #[must_use]
    pub fn new(data: Vec<T>, page: u32, page_size: u32, total_items: u64) -> Self {
        Self {
            data,
            pagination: PageInfo::new(page, page_size, total_items),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_page_info_calculation() {
        let page_info = PageInfo::new(1, 10, 25);
        assert_eq!(page_info.total_pages, 3);
        assert!(page_info.has_next());
        assert!(!page_info.has_previous());
    }

    #[test]
    fn test_paginated_serialization() {
        let paginated = Paginated::new(vec!["item1", "item2"], 1, 10, 2);
        let json = serde_json::to_string(&paginated).expect("Failed to serialize");
        assert!(json.contains("\"data\":[\"item1\",\"item2\"]"));
        assert!(json.contains("\"pageSize\":10"));
    }
}
