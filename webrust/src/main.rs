extern crate reqwest;
extern crate scraper;
extern crate serde_json;
extern crate serde_yaml;
extern crate toml;
extern crate csv;
extern crate xml;
extern crate rmp;

use xml::writer::{ EventWriter, EmitterConfig, XmlEvent };
use scraper::Html;
use serde_json::{ Map, Value };
use std::collections::{ HashMap, HashSet };
use std::error::Error;
use std::fs::{ self, File };
use std::io;
use scraper::Selector;
use std::io::Write;

fn main() -> Result<(), Box<dyn Error>> {
    loop {
        
        println!("Please enter the URL of the webpage:");
        let mut url = String::new();
        io::stdin().read_line(&mut url)?;

        let url = url.trim(); // Trim leading/trailing whitespace

        println!("Choose the search option:");
        println!("1. Search for a specific word in the HTML content");
        println!("2. Search for a specific HTML element");
        println!("3, Extract table");
        println!("4. Exit");

        let mut search_option = String::new();
        io::stdin().read_line(&mut search_option)?;

        match search_option.trim() {
            "1" => {
                println!("Enter the word you want to search for in the HTML content:");
                let mut word_to_search = String::new();
                io::stdin().read_line(&mut word_to_search)?;

                // Remove leading/trailing whitespace and newline characters
                let word_to_search = word_to_search.trim().to_string();

                // Send a GET request to the specified URL
                let html = reqwest::blocking::get(url)?.text()?;

                // Filter the HTML content based on the word to search
                let matching_text = filter_html_content(&html, &word_to_search);

                // Continue with the rest of the code for output options
                println!(
                    "Do you want to output the results as JSON, CSV, TOML, XML, or YAML? (json/csv/toml/xml/yaml)"
                );
                let mut output_format = String::new();
                io::stdin().read_line(&mut output_format)?;

                println!("Enter the name of the output file (without extension):");
                let mut output_filename = String::new();
                io::stdin().read_line(&mut output_filename)?;

                let output_filename = output_filename.trim(); // Trim leading/trailing whitespace

                // Check if a file with the same name and extension already exists
                let output_filename_with_extension = format_output_filename(
                    &output_filename,
                    output_format.trim()
                );

                if file_exists(&output_filename_with_extension) {
                    // The file already exists, ask the user if they want to replace it
                    println!(
                        "The file '{}' already exists. Do you want to replace it? (yes/no)",
                        &output_filename_with_extension
                    );
                    let mut replace_response = String::new();
                    io::stdin().read_line(&mut replace_response)?;
                    if replace_response.trim().to_lowercase() != "yes" {
                        // User doesn't want to replace the file, so exit
                        return Ok(());
                    }
                }

                match output_format.trim().to_lowercase().as_str() {
                    "json" => {
                        save_matching_text_as_json(
                            &matching_text,
                            &output_filename_with_extension
                        )?;
                        println!(
                            "JSON data has been saved to '{}.json'.",
                            &output_filename_with_extension
                        );
                    }
                    "csv" => {
                        save_matching_text_as_csv(&matching_text, &output_filename_with_extension)?;
                        println!(
                            "CSV data has been saved to '{}.csv'.",
                            &output_filename_with_extension
                        );
                    }
                    "toml" => {
                        save_matching_text_as_toml(
                            &matching_text,
                            &output_filename_with_extension
                        )?;
                        println!(
                            "TOML data has been saved to '{}.toml'.",
                            &output_filename_with_extension
                        );
                    }
                    "yaml" => {
                        save_matching_text_as_yaml(
                            &matching_text,
                            &output_filename_with_extension
                        )?;
                        println!(
                            "YAML data has been saved to '{}.yaml'.",
                            &output_filename_with_extension
                        );
                    }
                    "xml" => {
                        save_matching_text_as_xml(&matching_text, &output_filename_with_extension)?;
                        println!(
                            "XML data has been saved to '{}.xml'.",
                            &output_filename_with_extension
                        );
                    }
                    _ => {
                        println!(
                            "Invalid output format specified. Please choose 'json', 'csv', 'toml', 'yaml', or 'xml'."
                        );
                    }
                }
            }
            "2" => {
                println!("Enter the HTML element you want to search for (e.g., 'p', 'h1', 'a'):");
                let mut html_element = String::new();
                io::stdin().read_line(&mut html_element)?;

                // Remove leading/trailing whitespace and newline characters
                let html_element = html_element.trim().to_string();

                // Send a GET request to the specified URL
                let html = reqwest::blocking::get(url)?.text()?;

                // Filter the HTML content based on the specified HTML element
                let matching_text = filter_html_element(&html, &html_element);

                println!(
                    "Do you want to output the results as JSON, CSV, TOML, XML, or YAML? (json/csv/toml/xml/yaml)"
                );
                let mut output_format = String::new();
                io::stdin().read_line(&mut output_format)?;

                // Prompt the user for the output file name
                println!("Enter the name of the output file (without extension):");
                let mut output_filename = String::new();
                io::stdin().read_line(&mut output_filename)?;

                let output_filename = output_filename.trim(); // Trim leading/trailing whitespace

                // Check if a file with the same name and extension already exists
                let output_filename_with_extension = format_output_filename(
                    &output_filename,
                    output_format.trim()
                );

                if file_exists(&output_filename_with_extension) {
                    // The file already exists, ask the user if they want to replace it
                    println!(
                        "The file '{}' already exists. Do you want to replace it? (yes/no)",
                        &output_filename_with_extension
                    );
                    let mut replace_response = String::new();
                    io::stdin().read_line(&mut replace_response)?;
                    if replace_response.trim().to_lowercase() != "yes" {
                        // User doesn't want to replace the file, so exit
                        return Ok(());
                    }
                }

                match output_format.trim().to_lowercase().as_str() {
                    "json" => {
                        save_matching_text_as_json(
                            &matching_text,
                            &output_filename_with_extension
                        )?;
                        println!(
                            "JSON data has been saved to '{}.json'.",
                            &output_filename_with_extension
                        );
                    }
                    "csv" => {
                        save_matching_text_as_csv(&matching_text, &output_filename_with_extension)?;
                        println!(
                            "CSV data has been saved to '{}.csv'.",
                            &output_filename_with_extension
                        );
                    }
                    "toml" => {
                        save_matching_text_as_toml(
                            &matching_text,
                            &output_filename_with_extension
                        )?;
                        println!(
                            "TOML data has been saved to '{}.toml'.",
                            &output_filename_with_extension
                        );
                    }
                    "yaml" => {
                        save_matching_text_as_yaml(
                            &matching_text,
                            &output_filename_with_extension
                        )?;
                        println!(
                            "YAML data has been saved to '{}.yaml'.",
                            &output_filename_with_extension
                        );
                    }
                    "xml" => {
                        save_matching_text_as_xml(&matching_text, &output_filename_with_extension)?;
                        println!(
                            "XML data has been saved to '{}.xml'.",
                            &output_filename_with_extension
                        );
                    }
                    _ => {
                        println!(
                            "Invalid output format specified. Please choose 'json', 'csv', 'toml', 'yaml', or 'xml'."
                        );
                    }
                }
            }
            "3" => {
                // Send a GET request to the specified URL
                let html = reqwest::blocking::get(url)?.text()?;

                // Find the table element using a CSS selector
                let table_selector = Selector::parse("table").unwrap();
                let document = Html::parse_document(&html);
                let table_element = document.select(&table_selector).next();

                match table_element {
                    Some(table) => {
                        // Extract table data
                        let table_data = extract_table_data(&table);

                        // Prompt the user for the output format and filename
                        println!(
                            "Do you want to output the table data as JSON, CSV, TOML, XML, YAML (json/csv/toml/yaml/xml)"
                        );

                        let mut output_format = String::new();
                        io::stdin().read_line(&mut output_format)?;

                        println!("Enter the name of the output file (without extension):");
                        let mut output_filename = String::new();
                        io::stdin().read_line(&mut output_filename)?;

                        let output_filename = output_filename.trim(); // Trim leading/trailing whitespace
                        let output_filename_with_extension = format_output_filename(
                            &output_filename,
                            output_format.trim()
                        );

                        if file_exists(&output_filename_with_extension) {
                            // The file already exists, ask the user if they want to replace it
                            println!(
                                "The file '{}' already exists. Do you want to replace it? (yes/no)",
                                &output_filename_with_extension
                            );
                            let mut replace_response = String::new();
                            io::stdin().read_line(&mut replace_response)?;
                            if replace_response.trim().to_lowercase() != "yes" {
                                // User doesn't want to replace the file, so exit
                                return Ok(());
                            }
                        }

                        match output_format.trim().to_lowercase().as_str() {
                            "json" => {
                                save_table_data_as_json(
                                    &table_data,
                                    &output_filename_with_extension
                                )?;
                                println!(
                                    "JSON data has been saved to '{}.json'.",
                                    &output_filename_with_extension
                                );
                            }
                            "csv" => {
                                save_table_data_as_csv(
                                    &table_data,
                                    &output_filename_with_extension
                                )?;
                                println!(
                                    "CSV data has been saved to '{}.csv'.",
                                    &output_filename_with_extension
                                );
                            }
                            "toml" => {
                                save_table_data_as_toml(
                                    &table_data,
                                    &output_filename_with_extension
                                )?;
                                println!(
                                    "TOML data has been saved to '{}.toml'.",
                                    &output_filename_with_extension
                                );
                            }
                            "yaml" => {
                                save_table_data_as_yaml(
                                    &table_data,
                                    &output_filename_with_extension
                                )?;
                                println!(
                                    "YAML data has been saved to '{}.yaml'.",
                                    &output_filename_with_extension
                                );
                            }
                            "xml" => {
                                save_table_data_as_xml(
                                    &table_data,
                                    &output_filename_with_extension
                                )?;
                                println!(
                                    "XML data has been saved to '{}.xml'.",
                                    &output_filename_with_extension
                                );
                            }
                            _ => {
                                println!(
                                    "Invalid output format specified. Please choose 'json', 'csv', 'toml', 'yaml', or 'xml'."
                                );
                            }
                        }
                    }
                    None => {
                        println!("No table found on the webpage.");
                    }
                }
            }

            "4" => {
                println!("Exiting the program.");
                break; // Exit the loop
            }

            _ => {
                println!("Invalid choice. Please choose '1' or '2' or '3' for the search option.");
            }
            
        }
        println!("Do you want to continue? (yes/no)");
        let mut continue_option = String::new();
        io::stdin().read_line(&mut continue_option)?;

        if continue_option.trim().to_lowercase() != "yes" {
            break; // Exit the loop
        }
    }
    println!("Exiting the program.");
    Ok(())
}

fn filter_html_content(html: &str, word_to_search: &str) -> Vec<String> {
    // Implement your filtering logic for the HTML content and word here
    let matching_lines: Vec<String> = html
        .lines()
        .filter(|line| line.contains(word_to_search))
        .map(String::from)
        .collect();

    matching_lines
}

fn filter_html_element(html: &str, html_element: &str) -> Vec<String> {
    let fragment = Html::parse_document(html);
    let selector = Selector::parse(html_element).unwrap();

    let matching_text: Vec<String> = fragment
        .select(&selector)
        .map(|element| element.html())
        .collect();

    matching_text
}

fn file_exists(filename: &str) -> bool {
    fs::metadata(filename).is_ok()
}

fn extract_class_from_element(element: &str) -> String {
    if let Some(start) = element.find("class=\"") {
        if let Some(end) = element[start + 7..].find("\"") {
            return element[start + 7..start + 7 + end].to_string();
        }
    }
    String::new()
}

fn extract_text_from_element(element: &str) -> String {
    let fragment = Html::parse_fragment(element);
    let mut text = String::new();

    for node in fragment.tree.root().descendants() {
        if let scraper::node::Node::Text(text_node) = &node.value() {
            // Remove leading/trailing whitespace and add a space between text segments
            let cleaned_text = text_node.text.trim();
            if !cleaned_text.is_empty() {
                if !text.is_empty() {
                    text.push(' '); // Add space between text segments
                }
                text.push_str(cleaned_text);
            }
        }
    }

    text
}

fn extract_table_data(table: &scraper::ElementRef) -> Vec<Vec<String>> {
    let mut table_data = vec![];

    // Select all rows in the table
    let row_selector = Selector::parse("tr").unwrap();
    for row in table.select(&row_selector) {
        let mut row_data = vec![];

        // Select all cells (td/th) in the row
        let cell_selector = Selector::parse("td,th").unwrap();
        for cell in row.select(&cell_selector) {
            row_data.push(cell.text().collect());
        }

        table_data.push(row_data);
    }

    table_data
}

fn save_matching_text_as_json(
    matching_text: &Vec<String>,
    json_filename: &str
) -> Result<(), Box<dyn Error>> {
    let json_data = create_json_from_text(&matching_text);
    let mut json_file = File::create(json_filename)?;
    json_file.write_all(serde_json::to_string_pretty(&json_data)?.as_bytes())?;
    Ok(())
}

fn create_json_from_text(matching_text: &Vec<String>) -> serde_json::Value {
    let mut json_map: Map<String, Value> = Map::new();
    let mut text_set: HashSet<String> = HashSet::new();

    for text in matching_text {
        let class = extract_class_from_element(text);
        let text_content = extract_text_from_element(text);

        if !class.is_empty() && !text_content.is_empty() {
            if !text_set.contains(&text_content) {
                text_set.insert(text_content.clone());

                let entry = json_map
                    .entry(class.clone())
                    .or_insert_with(|| Value::Array(vec![]).to_owned());

                if let Value::Array(ref mut array) = entry {
                    array.push(Value::String(text_content));
                }
            }
        }
    }

    Value::Object(json_map)
}

fn save_matching_text_as_csv(
    matching_text: &Vec<String>,
    csv_filename: &str
) -> Result<(), Box<dyn Error>> {
    let mut csv_writer = csv::Writer::from_path(csv_filename)?;

    let mut data_map: HashMap<String, Vec<String>> = HashMap::new();

    for text in matching_text {
        let class = extract_class_from_element(text);
        let text_content = extract_text_from_element(text);

        if !class.is_empty() && !text_content.is_empty() {
            let entry = data_map.entry(class.clone()).or_insert_with(Vec::new);
            entry.push(text_content);
        }
    }

    // Create a header row with the class names
    let header_row: Vec<String> = data_map.keys().cloned().collect();
    csv_writer.write_record(&header_row)?;

    // Determine the maximum number of rows needed based on the class with the most data
    let max_rows = data_map
        .values()
        .map(|data| data.len())
        .max()
        .unwrap_or(0);

    // Create data rows with associated data
    for row_index in 0..max_rows {
        let mut data_row: Vec<String> = Vec::new();

        for class in header_row.iter() {
            if let Some(data) = data_map.get(class) {
                let value = data.get(row_index).cloned().unwrap_or_default();
                data_row.push(value);
            } else {
                data_row.push(String::new());
            }
        }

        csv_writer.write_record(&data_row)?;
    }

    csv_writer.flush()?;
    Ok(())
}

fn save_matching_text_as_toml(
    matching_text: &Vec<String>,
    toml_filename: &str
) -> Result<(), Box<dyn Error>> {
    let mut data_map: HashMap<String, Vec<String>> = HashMap::new();

    for text in matching_text {
        let class = extract_class_from_element(text);
        let text_content = extract_text_from_element(text);

        if !class.is_empty() && !text_content.is_empty() {
            let entry = data_map.entry(class.clone()).or_insert_with(Vec::new);
            entry.push(text_content);
        }
    }

    let toml_data = toml::to_string(&data_map)?;
    let mut toml_file = File::create(toml_filename)?;
    toml_file.write_all(toml_data.as_bytes())?;
    Ok(())
}

fn save_matching_text_as_yaml(
    matching_text: &Vec<String>,
    yaml_filename: &str
) -> Result<(), Box<dyn Error>> {
    let mut data_map: HashMap<String, Vec<String>> = HashMap::new();

    for text in matching_text {
        let class = extract_class_from_element(text);
        let text_content = extract_text_from_element(text);

        if !class.is_empty() && !text_content.is_empty() {
            let entry = data_map.entry(class.clone()).or_insert_with(Vec::new);
            entry.push(text_content);
        }
    }

    let yaml_str = serde_yaml::to_string(&data_map)?;
    let mut yaml_file = File::create(yaml_filename)?;
    yaml_file.write_all(yaml_str.as_bytes())?;
    Ok(())
}

fn format_output_filename(output_filename: &str, extension: &str) -> String {
    format!("{}.{}", output_filename, extension)
}

fn save_matching_text_as_xml(
    matching_text: &Vec<String>,
    xml_filename: &str
) -> Result<(), Box<dyn Error>> {
    let mut data_map: HashMap<String, Vec<String>> = HashMap::new();

    for text in matching_text {
        let class = extract_class_from_element(text);
        let text_content = extract_text_from_element(text);

        if !class.is_empty() && !text_content.is_empty() {
            let entry = data_map.entry(class.clone()).or_insert_with(Vec::new);
            entry.push(text_content);
        }
    }

    let file = File::create(xml_filename)?;
    let mut writer = EventWriter::new_with_config(file, EmitterConfig::new().perform_indent(true));

    writer.write(XmlEvent::start_element("data"))?;

    for (class, content) in &data_map {
        writer.write(XmlEvent::start_element("entry"))?;
        writer.write(XmlEvent::start_element("class"))?;
        writer.write(XmlEvent::characters(class))?;
        writer.write(XmlEvent::end_element())?;
        writer.write(XmlEvent::start_element("content"))?;
        writer.write(XmlEvent::characters(&content.join("\n")))?;
        writer.write(XmlEvent::end_element())?;
        writer.write(XmlEvent::end_element())?;
    }

    writer.write(XmlEvent::end_element())?;
    writer.write(XmlEvent::characters("\n"))?;

    Ok(())
}

fn create_json_from_table(table_data: &Vec<Vec<String>>) -> serde_json::Value {
    let mut json_array: Vec<serde_json::Value> = Vec::new();

    for row in table_data {
        let mut json_object: serde_json::Map<String, serde_json::Value> = serde_json::Map::new();

        for (index, cell) in row.iter().enumerate() {
            json_object.insert(format!("column{}", index), serde_json::Value::String(cell.clone()));
        }

        json_array.push(serde_json::Value::Object(json_object));
    }

    serde_json::Value::Array(json_array)
}

fn save_table_data_as_json(
    table_data: &Vec<Vec<String>>,
    json_filename: &str
) -> Result<(), Box<dyn Error>> {
    let json_data = create_json_from_table(&table_data);
    let mut json_file = File::create(json_filename)?;
    json_file.write_all(serde_json::to_string_pretty(&json_data)?.as_bytes())?;
    Ok(())
}

fn save_table_data_as_csv(table_data: &Vec<Vec<String>>, csv_filename: &str) -> Result<(), Box<dyn Error>> {
    let mut csv_writer = csv::Writer::from_path(csv_filename)?;

    // Determine the maximum number of columns in the table
    let max_columns = table_data.iter().map(|row| row.len()).max().unwrap_or(0);

    // Extend rows with fewer columns to match the maximum number of columns
    let extended_table_data: Vec<Vec<String>> = table_data
        .iter()
        .map(|row| {
            let mut extended_row = row.clone();
            while extended_row.len() < max_columns {
                extended_row.push("".to_string()); // Fill missing columns with empty strings
            }
            extended_row
        })
        .collect();

    for row in extended_table_data {
        csv_writer.write_record(&row)?;
    }

    csv_writer.flush()?;
    Ok(())
}

fn save_table_data_as_toml(
    table_data: &Vec<Vec<String>>,
    toml_filename: &str
) -> Result<(), Box<dyn Error>> {
    let toml_data = toml::to_string(&table_data)?;
    let mut toml_file = File::create(toml_filename)?;
    toml_file.write_all(toml_data.as_bytes())?;
    Ok(())
}

fn save_table_data_as_yaml(
    table_data: &Vec<Vec<String>>,
    yaml_filename: &str
) -> Result<(), Box<dyn Error>> {
    let yaml_str = serde_yaml::to_string(&table_data)?;
    let mut yaml_file = File::create(yaml_filename)?;
    yaml_file.write_all(yaml_str.as_bytes())?;
    Ok(())
}

fn save_table_data_as_xml(
    table_data: &Vec<Vec<String>>,
    xml_filename: &str
) -> Result<(), Box<dyn Error>> {
    let mut file = File::create(xml_filename)?;
    let mut writer = EventWriter::new_with_config(file, EmitterConfig::new().perform_indent(true));

    writer.write(XmlEvent::start_element("table"))?;

    for row in table_data {
        writer.write(XmlEvent::start_element("row"))?;

        for cell in row {
            writer.write(XmlEvent::start_element("cell"))?;
            writer.write(XmlEvent::characters(cell));
            writer.write(XmlEvent::end_element())?;
        }

        writer.write(XmlEvent::end_element())?;
    }

    writer.write(XmlEvent::end_element())?;

    Ok(())
}
