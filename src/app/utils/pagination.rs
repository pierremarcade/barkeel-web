use serde::Deserialize;

#[derive(Deserialize)]
pub struct PaginationQuery {
    pub page: Option<usize>,
    pub per_page: Option<usize>,
}

pub struct Pagination {
    pub total_results: i64
}

impl Pagination {
    fn generate_page_numbers(&self) -> Vec<String> {
        let mut page_numbers = Vec::new();
        if self.total_results <= 10 {
            for i in 1..=self.total_results {
                page_numbers.push(i.to_string());
            }
        } else {

            let last_page = (self.total_results + 9) / 10; // Calculer le dernier numéro de page
            page_numbers.push("1".to_string());
            page_numbers.push("2".to_string());
            page_numbers.push("3".to_string());
            if last_page > 3 {
                page_numbers.push("...".to_string()); // Séparateur
                page_numbers.push((last_page - 2).to_string());
                page_numbers.push((last_page - 1).to_string());
                page_numbers.push(last_page.to_string());
            }
        }
        page_numbers
    }
}
