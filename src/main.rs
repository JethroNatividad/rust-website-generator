use std::fs::{create_dir_all, File};
use std::io;
use std::io::Write;
use std::path::Path;

// A program that generates a website skeleton
// Inputs: site name, author name, javascript?, css?
// Process: generate html template with author in meta tag, create folder and file for js and css, link to html.
// output: website skeleton

fn get_input<T: std::str::FromStr>(prompt: &str) -> T {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        match input.trim().parse() {
            Ok(value) => break value,
            Err(_) => println!("Invalid input. Please try again."),
        }
    }
}

fn get_y_or_n(prompt: &str) -> bool {
    loop {
        let input: String = get_input(prompt);
        match input.to_lowercase().as_str() {
            "y" => return true,
            "n" => return false,
            "" => return true,
            _ => println!("Invalid Input"),
        }
    }
}

fn create_file(path: &String, content: &String) {
    // Convert the file path to a Path instance
    let path = Path::new(path);

    // Extract the directory path
    if let Some(dir) = path.parent() {
        // Create the directory if it doesn't exist
        create_dir_all(dir).unwrap();
    }

    // Create the file
    let mut file = File::create(path).unwrap();
    writeln!(file, "{}", content).unwrap();
}

fn main() {
    // Prompt sitename, "Site name: "
    let sitename: String = get_input("Site name: ");
    // Prompt author, "Author: "
    let author: String = get_input("Author: ");
    // Prompt includeJs, "Include Javascript? (Y/N) (default y): "
    let include_js: bool = get_y_or_n("Include Javascript? (Y/N) (default y): ");
    // Prompt includeCss, "Include CSS? (Y/N) (default y): "
    let include_css: bool = get_y_or_n("Include CSS? (Y/N) (default y): ");

    let script: String = if include_js {
        "\n        <script src=\"./js/main.js\"></script>".to_string()
    } else {
        "".to_string()
    };

    let css: String = if include_css {
        "\n        <link rel=\"stylesheet\" href=\"./css/index.css\">".to_string()
    } else {
        "".to_string()
    };

    // Write the top level of HTML to sitename/index.html
    let index_content = format!(
        r#"<!DOCTYPE html>
<html lang="en">
    <head>
        <meta charset="UTF-8" />
        <meta name="viewport" content="width=device-width, initial-scale=1.0" />
        <meta name="author" content="{0}" />{1}
        <title>{2}</title>
    </head>
    <body>
        <h1>Hello World!</h1>{3}
    </body>
</html>"#,
        author, css, sitename, script
    );

    // Create file in ./sitename/index.html
    create_file(&format!("./{0}/index.html", sitename), &index_content);
    create_file(&format!("./{0}/css/index.css", sitename), &"".to_string());
    create_file(&format!("./{0}/js/main.js", sitename), &"".to_string());

    println!("{}", index_content);
}
