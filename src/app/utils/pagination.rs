use serde::Deserialize;

#[derive(Deserialize)]
pub struct PaginationQuery {
    pub page: Option<usize>,
    pub per_page: Option<usize>,
}

pub struct Pagination {
    pub current_page: usize,
    pub per_page: usize,
    pub offset: usize,
    pub total_pages: i64,
    pub total_results: i64,
}

impl Pagination {
    pub fn new(pagination_query: PaginationQuery, total_results: i64) -> Self {
        let current_page = pagination_query.page.unwrap_or(1);
        let per_page = pagination_query.per_page.unwrap_or(10);
        let offset = (current_page - 1) * per_page;
        let total_pages = (total_results as f64 / per_page as f64).ceil() as i64;
        Pagination {
            current_page,
            per_page,
            offset,
            total_pages,
            total_results,
        }
    }
    pub fn generate_page_numbers(&self) -> Vec<String> {
        let mut page_numbers = Vec::new();
        if self.total_pages <= 10 {
            for i in 1..=self.total_pages {
                page_numbers.push(i.to_string());
            }
        } else {
            if (self.current_page as i64) < self.total_pages - 7 {
                if self.current_page % 2 == 1 {
                    page_numbers.push(self.current_page.to_string());
                    page_numbers.push((self.current_page + 1).to_string());
                    page_numbers.push((self.current_page + 2).to_string());
                    page_numbers.push("...".to_string());
                    page_numbers.push((self.total_pages - 2).to_string());
                    page_numbers.push((self.total_pages - 1).to_string());
                    page_numbers.push((self.total_pages).to_string());
                } else {
                    page_numbers.push((self.current_page - 1).to_string());
                    page_numbers.push(self.current_page.to_string());
                    page_numbers.push((self.current_page + 1).to_string());
                    page_numbers.push("...".to_string());
                    page_numbers.push((self.total_pages - 2).to_string());
                    page_numbers.push((self.total_pages - 1).to_string());
                    page_numbers.push((self.total_pages).to_string());
                }
            } else {
                for i in (self.total_pages - 10)..=self.total_pages {
                    page_numbers.push(i.to_string());
                }
            }
        }
        page_numbers
    }
}
