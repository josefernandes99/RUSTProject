use std::io;
use std::io::Read;
use rand::Rng;
use std::collections::HashMap;
use std::fmt;
use std::f64::consts::PI;

fn main() -> () {
    loop {
        clearscreen::clear().unwrap();

        println!("Current codes available:\n");
        println!("1. Calculator");
        println!("2. Guess the number!");
        println!("3. Fibonacci");
        println!("4. Prime Checker");
        println!("5. Array Mutation");
        println!("6. Random Array Mutation");
        println!("7. Flatten, Deduplicate, Filter");
        println!("8. String Editing");
        println!("9. String Ciphering");
        println!("10. Library");
        println!("11. Grocery Store");
        println!("12. Geometric Shapes");
        println!("\nWhich code do you want?: ");

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let parsed_num: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a number.");
                continue;
            }
        };

        match parsed_num {
            1 => {
                clearscreen::clear().unwrap();
                calculator();
            }
            2 => {
                clearscreen::clear().unwrap();
                guess_number();
            }
            3 => {
                clearscreen::clear().unwrap();
                fibonacci();
            }
            4 => {
                clearscreen::clear().unwrap();
                prime_checker();
            }
            5 => {
                clearscreen::clear().unwrap();
                array_mutation();
            }
            6 => {
                clearscreen::clear().unwrap();
                random_array_mutation();
            }
            7 => {
                clearscreen::clear().unwrap();
                flatten_deduplicate_filter_main();
            }
            8 => {
                clearscreen::clear().unwrap();
                string_editing();
            }
            9 => {
                clearscreen::clear().unwrap();
                string_ciphering();
            }
            10 => {
                clearscreen::clear().unwrap();
                library();
            }
            11 => {
                clearscreen::clear().unwrap();
                grocery_store();
            }
            12 => {
                clearscreen::clear().unwrap();
                geometric_shapes();
            }
            _ => println!("Invalid option. Please try again."),
        }
    }
}

fn press_to_continue() -> () {
    println!("\nPress ANY to continue...");
    let buffer = &mut [0u8];
    io::stdin().read_exact(buffer).unwrap();
}

//---------------------------------------------------
// PRIMEIROS CONCEITOS
//---------------------------------------------------

fn calculator() -> () {
    println!("Operations available:");
    println!("1. Addition");
    println!("2. Subtraction");
    println!("3. Division");
    println!("4. Multiplication");
    println!("\nWhich type of operation would you like to do?: ");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let parsed_num: u32 = input.trim().parse().expect("Please enter a valid number");

    match parsed_num {
        1 => addition(),
        2 => subtraction(),
        3 => division(),
        4 => multiplication(),
        _ => println!("Invalid operation."),
    }
}

fn addition() -> () {
    println!("What is the first number of the addition operation?: ");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let first_num: f32 = input.trim().parse().expect("Please enter a valid number");

    println!("What is the second number of the addition operation?: ");

    input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let second_num: f32 = input.trim().parse().expect("Please enter a valid number");

    let final_num: f32 = first_num + second_num;

    println!("The result of {} + {} is {}", first_num, second_num, final_num);

    press_to_continue()
}

fn subtraction() -> () {
    println!("What is the first number of the subtraction operation?: ");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let first_num: f32 = input.trim().parse().expect("Please enter a valid number");

    println!("What is the second number of the subtraction operation?: ");

    input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let second_num: f32 = input.trim().parse().expect("Please enter a valid number");

    let final_num: f32 = first_num - second_num;

    println!("The result of {} - {} is {}", first_num, second_num, final_num);

    press_to_continue()
}

fn division() -> () {
    println!("What is the first number of the division operation?: ");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let first_num: f32 = input.trim().parse().expect("Please enter a valid number");

    println!("What is the second number of the division operation?: ");

    input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let second_num: f32 = input.trim().parse().expect("Please enter a valid number");

    let final_num: f32 = first_num / second_num;

    println!("The result of {} ÷ {} is {}", first_num, second_num, final_num);

    press_to_continue()
}

fn multiplication() -> () {
    println!("What is the first number of the multiplication operation?: ");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let first_num: f32 = input.trim().parse().expect("Please enter a valid number");

    println!("What is the second number of the multiplication operation?: ");

    input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let second_num: f32 = input.trim().parse().expect("Please enter a valid number");

    let final_num: f32 = first_num * second_num;

    println!("The result of {} x {} is {}", first_num, second_num, final_num);

    press_to_continue()
}

fn guess_number() -> () {
    println!("Try to guess my number: ");

    let rand_num: u32 = rand::thread_rng().gen_range(1..=10);

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let parsed_num: u32 = input.trim().parse().expect("Please enter a valid number");

    if parsed_num == rand_num {
        println!("Correct number!\n");
    } else {
        println!("Wrong number! The correct number was: {}", rand_num);
    }

    press_to_continue()
}

fn fibonacci() -> () {
    println!("How many Fibonacci sequence numbers do you want?: ");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let parsed_num: u32 = input.trim().parse().expect("Please enter a valid number");

    let mut first_num: u32 = 0;
    let mut second_num: u32 = 1;

    println!("\nHere's the Fibonacci sequence:");

    if parsed_num == 1 {
        println!("{}", first_num);
    } else if parsed_num == 2 {
        println!("{}, {}", first_num, second_num);
    } else {
        let mut temp_num: u32;
        print!("{}, {}, ", first_num, second_num);
        for _ in 0..(parsed_num - 3) {
            temp_num = first_num + second_num;
            first_num = second_num;
            second_num = temp_num;
            print!("{}, ", temp_num);
        }
        temp_num = first_num + second_num;
        println!("{}", temp_num);
    }

    press_to_continue()
}

fn prime_checker() -> () {
    println!("What number do you want to check for primality?: ");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let parsed_num: u32 = input.trim().parse().expect("Please enter a valid number");

    let mut flag: u32 = 0;

    if parsed_num > 1 {
        for i in 2..(parsed_num - 1) {
            if parsed_num % i == 0 {
                flag = 1;
                break;
            }
        }
    } else {
        flag = 1;
    }

    if flag == 0 {
        println!("\nThe number {} is prime.", parsed_num);
    } else {
        println!("\nThe number {} is not prime.", parsed_num);
    }

    press_to_continue()
}

//---------------------------------------------------
// MODELO DE OWNERSHIP
//---------------------------------------------------

fn array_mutation() -> () {
    let mut array: [f32; 5] = [1.0, 2.0, 3.0, 4.0, 5.0];
    let array_ref: &mut [f32; 5] = &mut array;

    println!("Array: [1, 2, 3, 4, 5]\n");
    println!("Operations available:");
    println!("1. Addition");
    println!("2. Subtraction");
    println!("3. Division");
    println!("4. Multiplication");
    println!("\nWhich type of operation would you like to do?: ");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let parsed_num: u32 = input.trim().parse().expect("Please enter a valid number");

    println!("\nWhat is the number for this array mutation?: ");

    input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let number_selected: f32 = input.trim().parse().expect("Please enter a valid number");

    match parsed_num {
        1 => array_ref.iter_mut().for_each(|x| *x += number_selected),
        2 => array_ref.iter_mut().for_each(|x| *x -= number_selected),
        3 => array_ref.iter_mut().for_each(|x| *x /= number_selected),
        4 => array_ref.iter_mut().for_each(|x| *x *= number_selected),
        _ => println!("Invalid option."),
    }

    println!("\nArray after operation is {:?}", *array_ref);

    press_to_continue()
}

fn string_editing() {
    println!("Enter a string:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut input = input.trim().to_string();

    loop {
        clearscreen::clear().unwrap();
        println!("Current String: {}", input);
        println!(
            "
Choose an option:
1) Remove characters or strings
2) Add characters or strings
3) Convert to upper case
4) Convert to lower case
5) Back to main menu"
        );

        let mut option = String::new();
        io::stdin().read_line(&mut option).expect("Failed to read line");
        let option: u32 = match option.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a number.");
                continue;
            }
        };

        match option {
            1 => {
                println!("Enter a substring to remove:");
                let mut sub = String::new();
                io::stdin().read_line(&mut sub).expect("Failed to read line");
                let sub = sub.trim();
                input = input.replace(sub, "");
                println!("Result after removal: {}", input);
            }
            2 => {
                println!("Enter a substring to add:");
                let mut add = String::new();
                io::stdin().read_line(&mut add).expect("Failed to read line");
                let add = add.trim();
                input = format!("{}{}", input, add);
                println!("Result after adding: {}", input);
            }
            3 => input = input.to_uppercase(),
            4 => input = input.to_lowercase(),
            5 => break,
            _ => println!("Invalid option, please try again."),
        }
    }
}

fn string_ciphering() {
    println!("Enter the string to cipher:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input = input.trim().to_string();

    println!("Enter the key:");
    let mut key = String::new();
    io::stdin().read_line(&mut key).expect("Failed to read line");
    key = key.trim().to_string();

    let result = vigenere_cipher(&input, &key);
    println!("\nCiphered string: {}", result);

    press_to_continue();
}

fn vigenere_cipher(text: &str, key: &str) -> String {
    let key_bytes = key.as_bytes();
    let key_len = key.len();
    let mut result = String::new();

    for (i, c) in text.chars().enumerate() {
        let offset = key_bytes[i % key_len] % 128; // ASCII range wrapping
        let new_char = ((c as u8 + offset) % 128) as char;
        result.push(new_char);
    }

    result
}

//---------------------------------------------------
// STRUCTS + ESTRUTURAS DE DADOS
//---------------------------------------------------

#[derive(Debug)]
struct Book {
    title: String,
    author: String,
    isbn: String,
    keywords: Vec<String>,
    requested: bool,
}

fn library() {
    let mut books: Vec<Book> = Vec::new();

    loop {
        clearscreen::clear().unwrap();
        println!(
            "Library Menu:
1) Add Book
2) Remove Book
3) Request Book
4) Return Book
5) List All Books
6) Search by Author
7) Search by Keyword
8) Exit"
        );

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a number.");
                continue;
            }
        };

        match choice {
            1 => {
                println!("Enter book title:");
                let mut title = String::new();
                io::stdin().read_line(&mut title).expect("Failed to read line");

                println!("Enter book author:");
                let mut author = String::new();
                io::stdin().read_line(&mut author).expect("Failed to read line");

                println!("Enter book ISBN:");
                let mut isbn = String::new();
                io::stdin().read_line(&mut isbn).expect("Failed to read line");

                println!("Enter keywords (comma-separated):");
                let mut keywords_input = String::new();
                io::stdin().read_line(&mut keywords_input).expect("Failed to read line");
                let keywords: Vec<String> = keywords_input
                    .trim()
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .collect();

                books.push(Book {
                    title: title.trim().to_string(),
                    author: author.trim().to_string(),
                    isbn: isbn.trim().to_string(),
                    keywords,
                    requested: false,
                });
                println!("Book added successfully!");
                press_to_continue()
            }
            2 => {
                println!("Enter book title to remove:");
                let mut title = String::new();
                io::stdin().read_line(&mut title).expect("Failed to read line");
                let title = title.trim();

                if let Some(pos) = books.iter().position(|b| b.title == title && !b.requested) {
                    books.remove(pos);
                    println!("Book removed successfully!");
                } else {
                    println!("Book not found or it is currently requested.");
                }

                press_to_continue()
            }
            3 => {
                println!("Enter book title to request:");
                let mut title = String::new();
                io::stdin().read_line(&mut title).expect("Failed to read line");
                let title = title.trim();

                if let Some(book) = books.iter_mut().find(|b| b.title == title) {
                    book.requested = true;
                    println!("Book requested successfully!");
                } else {
                    println!("Book not found.");
                }

                press_to_continue()
            }
            4 => {
                println!("Enter book title to return:");
                let mut title = String::new();
                io::stdin().read_line(&mut title).expect("Failed to read line");
                let title = title.trim();

                if let Some(book) = books.iter_mut().find(|b| b.title == title) {
                    book.requested = false;
                    println!("Book returned successfully!");
                } else {
                    println!("Book not found.");
                }

                press_to_continue()
            }
            5 => {
                if books.is_empty() {
                    println!("No books in the library.");
                } else {
                    println!("Listing all books:");
                    for book in &books {
                        println!(
                            "Title: {}, Author: {}, ISBN: {}, Keywords: {:?}, Requested: {}",
                            book.title, book.author, book.isbn, book.keywords, book.requested
                        );
                    }
                }
                press_to_continue()
            }
            6 => {
                println!("Enter author name to search:");
                let mut author = String::new();
                io::stdin().read_line(&mut author).expect("Failed to read line");
                let author = author.trim();

                let results: Vec<&Book> = books.iter().filter(|b| b.author == author).collect();
                if results.is_empty() {
                    println!("No books found by author: {}", author);
                } else {
                    println!("Books by {}:", author);
                    for book in results {
                        println!(
                            "Title: {}, ISBN: {}, Keywords: {:?}, Requested: {}",
                            book.title, book.isbn, book.keywords, book.requested
                        );
                    }
                }
                press_to_continue()
            }
            7 => {
                println!("Enter keyword:");
                let mut keyword = String::new();
                io::stdin().read_line(&mut keyword).expect("Failed to read line");
                let keyword = keyword.trim().to_string();

                println!(
                    "Choose keyword search type:
1) Union (any matching keyword)
2) Intersection (all matching keywords)"
                );
                let mut search_type = String::new();
                io::stdin()
                    .read_line(&mut search_type)
                    .expect("Failed to read line");
                let search_type: u32 = search_type.trim().parse().expect("Invalid number");

                match search_type {
                    1 => {
                        let results: Vec<&Book> = books
                            .iter()
                            .filter(|b| b.keywords.contains(&keyword))
                            .collect();
                        if results.is_empty() {
                            println!("No books found with keyword: {}", keyword);
                        } else {
                            println!("Books with keyword '{}':", keyword);
                            for book in results {
                                println!(
                                    "Title: {}, Author: {}, ISBN: {}, Requested: {}",
                                    book.title, book.author, book.isbn, book.requested
                                );
                            }
                        }
                    }
                    2 => {
                        println!("Enter additional keywords (comma-separated):");
                        let mut additional_keywords = String::new();
                        io::stdin()
                            .read_line(&mut additional_keywords)
                            .expect("Failed to read line");
                        let additional_keywords: Vec<String> = additional_keywords
                            .trim()
                            .split(',')
                            .map(|s| s.trim().to_string())
                            .collect();

                        let results: Vec<&Book> = books
                            .iter()
                            .filter(|b| {
                                let all_keywords: Vec<String> =
                                    std::iter::once(keyword.clone()).chain(additional_keywords.clone()).collect();
                                all_keywords.iter().all(|k| b.keywords.contains(k))
                            })
                            .collect();

                        if results.is_empty() {
                            println!("No books found matching all keywords.");
                        } else {
                            println!("Books matching all keywords:");
                            for book in results {
                                println!(
                                    "Title: {}, Author: {}, ISBN: {}, Requested: {}",
                                    book.title, book.author, book.isbn, book.requested
                                );
                            }
                        }
                    }
                    _ => println!("Invalid search type."),
                }
                press_to_continue()
            }
            8 => break,
            _ => println!("Invalid option, please try again."),
        }
    }
}

#[derive(Debug)]
struct Product {
    id: u32,
    name: String,
    expiring_date: String,
    price: f32,
    quantity: u32,
}

#[derive(Debug)]
struct Level {
    products: HashMap<u32, Product>,
}

#[derive(Debug)]
struct Shelf {
    levels: HashMap<u32, Level>,
}

#[derive(Debug)]
struct Row {
    shelves: HashMap<u32, Shelf>,
}

#[derive(Debug)]
struct GroceryStore {
    rows: HashMap<u32, Row>,
}

impl fmt::Display for Row {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.shelves.len())
    }
}

impl fmt::Display for Shelf {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.levels.len())
    }
}

impl fmt::Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.products.len())
    }
}

impl GroceryStore {
    fn new() -> Self {
        GroceryStore {
            rows: HashMap::new(),
        }
    }

    fn add_product(&mut self, row: u32, shelf: u32, level: u32, product: Product) {
        self.rows
            .entry(row)
            .or_insert_with(|| Row {
                shelves: HashMap::new(),
            })
            .shelves
            .entry(shelf)
            .or_insert_with(|| Shelf {
                levels: HashMap::new(),
            })
            .levels
            .entry(level)
            .or_insert_with(|| Level {
                products: HashMap::new(),
            })
            .products
            .insert(product.id, product);
        println!("Product added successfully!");
    }

    fn remove_product(&mut self, row: u32, shelf: u32, level: u32, product_id: u32) {
        if let Some(row) = self.rows.get_mut(&row) {
            if let Some(shelf) = row.shelves.get_mut(&shelf) {
                if let Some(level) = shelf.levels.get_mut(&level) {
                    if level.products.remove(&product_id).is_some() {
                        println!("Product removed successfully!");
                    } else {
                        println!("Product not found.");
                    }
                }
            }
        }
    }

    fn move_product(&mut self, from: (u32, u32, u32), to: (u32, u32, u32), product_id: u32) {
        if let Some(product) = self
            .rows
            .get_mut(&from.0)
            .and_then(|row| row.shelves.get_mut(&from.1))
            .and_then(|shelf| shelf.levels.get_mut(&from.2))
            .and_then(|level| level.products.remove(&product_id))
        {
            self.add_product(to.0, to.1, to.2, product);
            println!("Product moved successfully!");
        } else {
            println!("Product not found at the specified location.");
        }
    }

    fn change_price(&mut self, row: u32, shelf: u32, level: u32, product_id: u32, new_price: f32) {
        if let Some(row) = self.rows.get_mut(&row) {
            if let Some(shelf) = row.shelves.get_mut(&shelf) {
                if let Some(level) = shelf.levels.get_mut(&level) {
                    if let Some(product) = level.products.get_mut(&product_id) {
                        product.price = new_price;
                        println!("Price updated successfully!");
                    } else {
                        println!("Product not found.");
                    }
                }
            }
        }
    }

    fn change_name(&mut self, row: u32, shelf: u32, level: u32, product_id: u32, new_name: String) {
        if let Some(row) = self.rows.get_mut(&row) {
            if let Some(shelf) = row.shelves.get_mut(&shelf) {
                if let Some(level) = shelf.levels.get_mut(&level) {
                    if let Some(product) = level.products.get_mut(&product_id) {
                        product.name = new_name;
                        println!("Name updated successfully!");
                    } else {
                        println!("Product not found.");
                    }
                }
            }
        }
    }

    fn add_quantity(&mut self, row: u32, shelf: u32, level: u32, product_id: u32, quantity: u32) {
        if let Some(row) = self.rows.get_mut(&row) {
            if let Some(shelf) = row.shelves.get_mut(&shelf) {
                if let Some(level) = shelf.levels.get_mut(&level) {
                    if let Some(product) = level.products.get_mut(&product_id) {
                        product.quantity += quantity;
                        println!("Quantity added successfully!");
                    } else {
                        println!("Product not found.");
                    }
                }
            }
        }
    }

    fn remove_quantity(&mut self, row: u32, shelf: u32, level: u32, product_id: u32, quantity: u32) {
        if let Some(row) = self.rows.get_mut(&row) {
            if let Some(shelf) = row.shelves.get_mut(&shelf) {
                if let Some(level) = shelf.levels.get_mut(&level) {
                    if let Some(product) = level.products.get_mut(&product_id) {
                        if product.quantity >= quantity {
                            product.quantity -= quantity;
                            println!("Quantity removed successfully!");
                        } else {
                            println!("Not enough quantity to remove.");
                        }
                    } else {
                        println!("Product not found.");
                    }
                }
            }
        }
    }
    fn list_all_products(&self) {
        if self.rows.is_empty() {
            println!("No products in the store.");
            return;
        }

        println!("Listing all products in the store:");
        for (row_num, row) in &self.rows {
            for (shelf_num, shelf) in &row.shelves {
                for (level_num, level) in &shelf.levels {
                    for product in level.products.values() {
                        println!(
                            "Row: {}, Shelf: {}, Level: {}, ID: {}, Name: {}, Expiring Date: {}, Price: {:.2}, Quantity: {}",
                            row_num, shelf_num, level_num, product.id, product.name, product.expiring_date, product.price, product.quantity
                        );
                    }
                }
            }
        }
    }
    fn list_from_row(&self, row: u32) {
        if let Some(row) = self.rows.get(&row) {
            println!("Products in Row {}:", row);
            for (shelf_num, shelf) in &row.shelves {
                for (level_num, level) in &shelf.levels {
                    for product in level.products.values() {
                        println!(
                            "Shelf: {}, Level: {}, ID: {}, Name: {}, Expiring Date: {}, Price: {:.2}, Quantity: {}",
                            shelf_num, level_num, product.id, product.name, product.expiring_date, product.price, product.quantity
                        );
                    }
                }
            }
        } else {
            println!("Row {} not found.", row);
        }
    }

    fn list_from_shelf(&self, row: u32, shelf: u32) {
        if let Some(row) = self.rows.get(&row) {
            if let Some(shelf) = row.shelves.get(&shelf) {
                println!("Products in Row {}, Shelf {}:", row, shelf);
                for (level_num, level) in &shelf.levels {
                    for product in level.products.values() {
                        println!(
                            "Level: {}, ID: {}, Name: {}, Expiring Date: {}, Price: {:.2}, Quantity: {}",
                            level_num, product.id, product.name, product.expiring_date, product.price, product.quantity
                        );
                    }
                }
            } else {
                println!("Shelf {} not found in Row {}.", shelf, row);
            }
        } else {
            println!("Row {} not found.", row);
        }
    }

    fn list_from_level(&self, row: u32, shelf: u32, level: u32) {
        if let Some(row) = self.rows.get(&row) {
            if let Some(shelf) = row.shelves.get(&shelf) {
                if let Some(level) = shelf.levels.get(&level) {
                    println!("Products in Row {}, Shelf {}, Level {}:", row, shelf, level);
                    for product in level.products.values() {
                        println!(
                            "ID: {}, Name: {}, Expiring Date: {}, Price: {:.2}, Quantity: {}",
                            product.id, product.name, product.expiring_date, product.price, product.quantity
                        );
                    }
                } else {
                    println!("Level {} not found in Shelf {}.", level, shelf);
                }
            } else {
                println!("Shelf {} not found in Row {}.", shelf, row);
            }
        } else {
            println!("Row {} not found.", row);
        }
    }

    fn list_by_name(&self, name: &str) {
        let mut found = false;
        for row in self.rows.values() {
            for shelf in row.shelves.values() {
                for level in shelf.levels.values() {
                    for product in level.products.values() {
                        if product.name == name {
                            println!(
                                "Row: {}, Shelf: {}, Level: {}, ID: {}, Expiring Date: {}, Price: {:.2}, Quantity: {}",
                                row, shelf, level, product.id, product.expiring_date, product.price, product.quantity
                            );
                            found = true;
                        }
                    }
                }
            }
        }
        if !found {
            println!("No products found with name '{}'.", name);
        }
    }
}

fn grocery_store() {
    let mut store = GroceryStore::new();

    loop {
        clearscreen::clear().unwrap();
        println!(
            "Grocery Store Menu:
1) Add Product
2) Remove Product
3) Move Product
4) Change Price
5) Change Name
6) Add Quantity
7) Remove Quantity
8) List All Products
9) List from Row
10) List from Shelf
11) List from Level
12) List by Name
13) Exit"
        );

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a number.");
                continue;
            }
        };

        match choice {
            1 => {
                println!("Enter row, shelf, and level (space-separated):");
                let mut pos = String::new();
                io::stdin().read_line(&mut pos).expect("Failed to read line");
                let pos: Vec<u32> = pos
                    .trim()
                    .split_whitespace()
                    .map(|x| x.parse().expect("Invalid number"))
                    .collect();

                println!("Enter product ID, name, expiring date, price, and quantity (space-separated):");
                let mut product_info = String::new();
                io::stdin().read_line(&mut product_info).expect("Failed to read line");
                let product_info: Vec<&str> = product_info.trim().split(' ').collect();

                let product = Product {
                    id: product_info[0].parse().expect("Invalid ID"),
                    name: product_info[1].to_string(),
                    expiring_date: product_info[2].to_string(),
                    price: product_info[3].parse().expect("Invalid price"),
                    quantity: product_info[4].parse().expect("Invalid quantity"),
                };

                store.add_product(pos[0], pos[1], pos[2], product);
                press_to_continue();
            }
            2 => {
                println!("Enter row, shelf, level, and product ID:");
                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Failed to read line");
                let values: Vec<u32> = input
                    .trim()
                    .split_whitespace()
                    .map(|x| x.parse().expect("Invalid number"))
                    .collect();

                store.remove_product(values[0], values[1], values[2], values[3]);
                press_to_continue();
            }
            3 => {
                println!("Enter source (row shelf level) and destination (row shelf level) and product ID:");
                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Failed to read line");
                let values: Vec<u32> = input
                    .trim()
                    .split_whitespace()
                    .map(|x| x.parse().expect("Invalid number"))
                    .collect();

                store.move_product(
                    (values[0], values[1], values[2]),
                    (values[3], values[4], values[5]),
                    values[6],
                );
                press_to_continue();
            }
            4 => {
                println!("Enter row, shelf, level, product ID, and new price:");
                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Failed to read line");
                let values: Vec<f32> = input
                    .trim()
                    .split_whitespace()
                    .map(|x| x.parse().expect("Invalid number"))
                    .collect();

                store.change_price(
                    values[0] as u32,
                    values[1] as u32,
                    values[2] as u32,
                    values[3] as u32,
                    values[4],
                );
                press_to_continue();
            }
            5 => {
                println!("Enter row, shelf, level, product ID, and new name (space-separated):");
                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Failed to read line");
                let values: Vec<&str> = input.trim().split(' ').collect();

                store.change_name(
                    values[0].parse().unwrap(),
                    values[1].parse().unwrap(),
                    values[2].parse().unwrap(),
                    values[3].parse().unwrap(),
                    values[4].to_string(),
                );
                press_to_continue();
            }
            6 => {
                println!("Enter row, shelf, level, product ID, and quantity to add:");
                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Failed to read line");
                let values: Vec<u32> = input
                    .trim()
                    .split_whitespace()
                    .map(|x| x.parse().unwrap())
                    .collect();

                store.add_quantity(values[0], values[1], values[2], values[3], values[4]);
                press_to_continue();
            }
            7 => {
                println!("Enter row, shelf, level, product ID, and quantity to remove:");
                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Failed to read line");
                let values: Vec<u32> = input
                    .trim()
                    .split_whitespace()
                    .map(|x| x.parse().unwrap())
                    .collect();

                store.remove_quantity(values[0], values[1], values[2], values[3], values[4]);
                press_to_continue();
            }
            8 => {
                store.list_all_products();
                press_to_continue();
            }
            9 => {
                println!("Enter row number:");
                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Failed to read line");
                let row: u32 = input.trim().parse().expect("Invalid number");

                store.list_from_row(row);
                press_to_continue();
            }
            10 => {
                println!("Enter row and shelf numbers (space-separated):");
                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Failed to read line");
                let values: Vec<u32> = input
                    .trim()
                    .split_whitespace()
                    .map(|x| x.parse().unwrap())
                    .collect();

                store.list_from_shelf(values[0], values[1]);
                press_to_continue();
            }
            11 => {
                println!("Enter row, shelf, and level numbers (space-separated):");
                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Failed to read line");
                let values: Vec<u32> = input
                    .trim()
                    .split_whitespace()
                    .map(|x| x.parse().unwrap())
                    .collect();

                store.list_from_level(values[0], values[1], values[2]);
                press_to_continue();
            }
            12 => {
                println!("Enter product name:");
                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Failed to read line");
                let name = input.trim();

                store.list_by_name(name);
                press_to_continue();
            }
            13 => break,
            _ => println!("Invalid option, please try again."),
        }
    }
}

fn random_array_mutation() {
    use rand::Rng;

    struct ArrayContainer {
        array: Vec<f32>,
    }

    impl ArrayContainer {
        fn new(size: usize) -> Self {
            let mut rng = rand::thread_rng();
            let array = (0..size).map(|_| rng.gen_range(1.0..100.0)).collect();
            ArrayContainer { array }
        }

        fn apply_operation<F>(&mut self, operation: F)
        where
            F: Fn(f32) -> f32,
        {
            self.array = self.array.iter().map(|&x| operation(x)).collect();
        }

        fn display(&self) {
            println!("Current Array: {:?}", self.array);
        }
    }

    println!("Enter the size of the array:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let size: usize = input.trim().parse().expect("Invalid size");

    let mut container = ArrayContainer::new(size);
    container.display();

    println!(
        "
Choose an operation:
1) Add a number
2) Subtract a number
3) Multiply by a number
4) Divide by a number
"
    );

    input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let operation: u32 = input.trim().parse().expect("Invalid operation");

    println!("Enter the number to apply:");
    input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let number: f32 = input.trim().parse().expect("Invalid number");

    match operation {
        1 => container.apply_operation(|x| x + number),
        2 => container.apply_operation(|x| x - number),
        3 => container.apply_operation(|x| x * number),
        4 => container.apply_operation(|x| x / number),
        _ => println!("Invalid operation."),
    }

    container.display();
    press_to_continue();
}

fn flatten_deduplicate_filter(nested_vec: Vec<Vec<u32>>) -> Vec<u32> {
    use std::collections::HashSet;

    nested_vec
        .into_iter()                      // Flatten the nested vector
        .flatten()
        .filter(|&x| x % 2 == 0 || x % 3 == 0) // Keep only multiples of 2 or 3
        .collect::<HashSet<_>>()          // Remove duplicates using a HashSet
        .into_iter()
        .collect()                        // Convert back into a vector
}

fn flatten_deduplicate_filter_main() {
    println!("Enter nested vectors (comma-separated inner vectors, space-separated numbers):");
    println!("Example: 1 2 3, 4 5 6, 7 8 9");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let nested_vec: Vec<Vec<u32>> = input
        .trim()
        .split(',')
        .map(|s| s.trim().split_whitespace().map(|n| n.parse().unwrap()).collect())
        .collect();

    let result = flatten_deduplicate_filter(nested_vec);
    println!("Resulting vector (flattened, deduplicated, filtered): {:?}", result);

    press_to_continue();
}

//---------------------------------------------------
// ENUMS E MATCHING
//---------------------------------------------------

enum Shape {
    Square { side: f64 },
    Circle { radius: f64 },
    Ellipse { a: f64, b: f64 },
    Triangle { base: f64, height: f64, side1: f64, side2: f64 },
    Cube { side: f64 },
    Cylinder { radius: f64, height: f64 },
    Sphere { radius: f64 },
}

impl Shape {
    fn area(&self) -> Option<f64> {
        match self {
            Shape::Square { side } => Some(side * side),
            Shape::Circle { radius } => Some(PI * radius * radius),
            Shape::Ellipse { a, b } => Some(PI * a * b),
            Shape::Triangle { base, height, .. } => Some(0.5 * base * height),
            Shape::Cube { .. } => None,
            Shape::Cylinder { .. } => None,
            Shape::Sphere { .. } => None,
        }
    }

    fn perimeter(&self) -> Option<f64> {
        match self {
            Shape::Square { side } => Some(4.0 * side),
            Shape::Circle { radius } => Some(2.0 * PI * radius),
            Shape::Ellipse { a, b } => {
                let h = ((a - b).powi(2)) / ((a + b).powi(2));
                Some(PI * (a + b) * (1.0 + (3.0 * h) / (10.0 + (4.0 - 3.0 * h).sqrt())))
            }
            Shape::Triangle { base, side1, side2, .. } => Some(base + side1 + side2),
            Shape::Cube { .. } => None,
            Shape::Cylinder { .. } => None,
            Shape::Sphere { .. } => None,
        }
    }

    fn volume(&self) -> Option<f64> {
        match self {
            Shape::Square { .. } => None, // Squares don't have a volume
            Shape::Circle { .. } => None,  // Circles don't have a volume
            Shape::Ellipse { .. } => None, // Ellipses don't have a volume
            Shape::Triangle { .. } => None, // Triangles don't have a volume
            Shape::Cube { side } => Some(side.powi(3)),
            Shape::Cylinder { radius, height } => Some(PI * radius * radius * height),
            Shape::Sphere { radius } => Some((4.0 / 3.0) * PI * radius.powi(3)),
        }
    }
}

fn geometric_shapes() {
    loop {
        clearscreen::clear().unwrap();
        println!(
            "Choose a geometric shape:
1) Square
2) Circle
3) Ellipse
4) Triangle
5) Cube
6) Cylinder
7) Sphere
8) Exit"
        );

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let choice: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a number.");
                continue;
            }
        };

        let shape = match choice {
            1 => {
                println!("Enter the side length of the square:");
                let side = read_f64();
                Shape::Square { side }
            }
            2 => {
                println!("Enter the radius of the circle:");
                let radius = read_f64();
                Shape::Circle { radius }
            }
            3 => {
                println!("Enter the semi-major axis (a) of the ellipse:");
                let a = read_f64();
                println!("Enter the semi-minor axis (b) of the ellipse:");
                let b = read_f64();
                Shape::Ellipse { a, b }
            }
            4 => {
                println!("Enter the base of the triangle:");
                let base = read_f64();
                println!("Enter the height of the triangle:");
                let height = read_f64();
                println!("Enter the first side length:");
                let side1 = read_f64();
                println!("Enter the second side length:");
                let side2 = read_f64();
                Shape::Triangle { base, height, side1, side2 }
            }
            5 => {
                println!("Enter the side length of the cube:");
                let side = read_f64();
                Shape::Cube { side }
            }
            6 => {
                println!("Enter the radius of the cylinder:");
                let radius = read_f64();
                println!("Enter the height of the cylinder:");
                let height = read_f64();
                Shape::Cylinder { radius, height }
            }
            7 => {
                println!("Enter the radius of the sphere:");
                let radius = read_f64();
                Shape::Sphere { radius }
            }
            8 => break,
            _ => {
                println!("Invalid option, please try again.");
                continue;
            }
        };

        match shape.area() {
            Some(area) => println!("Area: {:.2}", area),
            None => println!("Area: Not applicable"),
        }
        match shape.perimeter() {
            Some(perimeter) => println!("Perimeter: {:.2}", perimeter),
            None => println!("Perimeter: Not applicable"),
        }
        match shape.volume() {
            Some(volume) => println!("Volume: {:.2}", volume),
            None => println!("Volume: Not applicable"),
        }
        press_to_continue();
    }
}

fn read_f64() -> f64 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().parse().expect("Invalid number")
}
