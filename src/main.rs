pub mod book_lists;
pub mod file_writer;
use reqwest;
use serde_json::Value;
use file_writer::write_to_file;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let books = book_lists::NEWSWEEK_100_BOOKS;
    let mut book_data: Vec<(String, String, i64, i64)> = Vec::new();

    for book in books.iter() {
        let url = format!(
            "https://www.googleapis.com/books/v1/volumes?q=intitle:{}",
            book
        );

        let resp = reqwest::get(&url).await?;
        let json: Value = resp.json().await?;

        let items = json["items"].as_array();

        if let Some(items) = items {
            let item = &items[0];
            let title = item["volumeInfo"]["title"].as_str().unwrap_or("N/A");
            let authors = item["volumeInfo"]["authors"].as_array()
                .map(|a| a.iter().map(|a| a.as_str().unwrap_or("")).collect::<Vec<&str>>().join(", "))
                .unwrap_or_else(|| "N/A".to_string());
            let page_count = item["volumeInfo"]["pageCount"].as_i64().unwrap_or(0);

            // For simplicity, we are converting page count to approximate word count by assuming 250 words per page.
            let word_count = page_count * 250;

            book_data.push((title.to_string(), authors.to_string(), word_count, page_count));

            /*println!(
                "Title: {}\nAuthors: {}\nApproximate Word Count: {}\nPages: {}\n",
                title, authors, word_count, page_count
            );*/
        }
    }

    if let Err(e) = write_to_file(book_data) {
        eprintln!("Error writing to file: {}", e);
    }    

    Ok(())
}

