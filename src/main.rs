use clap::Parser;
use maud::{DOCTYPE, Markup, html};
use pulldown_cmark::{Parser as markdownParser, Options, html};
use std::{fs, path::PathBuf};
use std::fs::read_dir;

#[derive(Parser, Debug)]

struct Args{

    /// Input markdown file
    #[arg(short, long)]
    input: PathBuf,

    /// Output markdown file
    #[arg(short, long)]
    output: Option<PathBuf>,
}

fn render_html_page(content: &str) -> Markup {
    html! {
        (DOCTYPE)
        html {
            head {
                meta charset="UTF-8";
                title { "Markdown to HTML Converter" }

                style {
                    "
                    body {
                        font-family: Arial, sans-serif;
                        margin: 40px;
                        max-width: 800px;
                        line-height: 1.6;
                    }
                    pre {
                        background: #f4f4f4;
                        padding: 10px;
                        overflow-x: auto;
                    }
                    code {
                        background: #eee;
                        padding: 2px 4px;
                    }
                    "
                }
            }
            body {
                (maud::PreEscaped(content.to_string()))
            }
        }
    }
}

fn convert_directory(input: &PathBuf, output: &PathBuf) {
    fs::create_dir_all(output).expect("Failed to create output directory");

    for entry in read_dir(input).expect("Failed to read directory") {
        let entry = entry.expect("Failed to read entry");
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) == Some("md") {
            let markdown = fs::read_to_string(&path).expect("Failed to read markdown");

            let parser = markdownParser::new_ext(&markdown, Options::all());

            let mut html_output = String::new();
            html::push_html(&mut html_output, parser);

            let html_page = render_html_page(&html_output).into_string();

            let mut output_file = output.join(path.file_stem().unwrap());
            output_file.set_extension("html");

            fs::write(output_file, html_page).expect("Failed to write html");
        }
    }
}


fn main() {
    let args = Args::parse();
    if args.input.is_dir() {
    let output_dir = args.output.unwrap_or_else(|| PathBuf::from("site"));
    convert_directory(&args.input, &output_dir);
    println!("Site generated successfully!");
    return;
}
    let markdown_input = match fs::read_to_string(&args.input) {
    Ok(content) => content,
    Err(e) => {
        eprintln!("Error reading file: {}", e);
        return;
    }
};

    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);

    let parser = markdownParser::new_ext(&markdown_input, options);

    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    let html_page = render_html_page(&html_output).into_string();

    let output_path = args.output.unwrap_or_else(|| PathBuf::from("output.html"));

    fs::write(output_path, html_page).expect("Failed to write output file");
}
