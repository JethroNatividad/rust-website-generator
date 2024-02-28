use std::io;
use std::io::Write;

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

fn main() {
    // Prompt sitename, "Site name: "
    // Prompt author, "Author: "
    // Prompt includeJs, "Include Javascript? (Y/N) (default y): "
    // Prompt includeCss, "Include CSS? (Y/N) (default y): "

    // Write the top level of HTML to sitename/index.html
    // <!DOCTYPE html>
    // <html lang="en">
    //   <head>
    //     <meta charset="UTF-8" />
    //     <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    //     <meta name="author" content={author} />

    // if includeCss
    // create file in sitename/css/index.css
    // then write to sitename/index.html
    // <link rel="stylesheet" href="./css/index.css">

    // Continue writing the rest
    //     <title>Document</title>
    //   </head>
    //   <body>

    // if includeJs
    // create file in sitename/js/main.js
    // then write to sitename/index.html
    // <script src="./js/main.js"></script>

    // then write the rest
    // </body>
    // </html>
}
