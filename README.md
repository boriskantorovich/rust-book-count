# Rust Google Books API Project

This project is a Rust application that queries the Google Books API to fetch details about a list of books, including the title, authors, and approximate word count. The data is then sorted by word count and written to a file.

## Features

- Fetches book details from Google Books API.
- Retrieves title, authors, and approximate word count for each book.
- Stores the results in a file, sorted by word count.

## Prerequisites

- Rust Programming Language
- `reqwest` crate for making API requests
- `serde_json` crate for parsing JSON data

## Usage

1. Clone the repository.
    ```bash
    git clone https://github.com/yourusername/yourproject.git
    ```
   
2. Navigate to the project folder.
    ```bash
    cd yourproject
    ```
   
3. Build the project.
    ```bash
    cargo build
    ```

4. Run the project.
    ```bash
    cargo run
    ```

## Structure

- `main.rs`: Entry point for the program.
- `book_lists.rs`: Contains the list of books to be queried.
- `file_writer.rs`: Handles writing the sorted results to a file.

## Future Improvements

- Handling rate limits and pagination from the Google Books API.
- More robust error handling.

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details.
