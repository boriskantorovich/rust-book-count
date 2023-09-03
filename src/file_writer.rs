use std::fs::File;
use std::io::Write;

pub fn write_to_file(data: Vec<(String, String, i64, i64)>) -> std::io::Result<()> {
    // Sort the data by word count in ascending order
    let mut sorted_data = data.clone();
	sorted_data.sort_by(|a, b| a.2.cmp(&b.2));
    
    // Open file
    let mut file = File::create("books.txt")?;
    
    // Write to file
	for (title, authors, word_count, page_count) in sorted_data.iter() {
        writeln!(file, "Title: {}\nAuthors: {}\nApproximate Word Count: {}\nPage Count: {}\n", title, authors, word_count, page_count)?;
    }
    
    Ok(())
}
