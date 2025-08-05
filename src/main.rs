use std::{io::{Read, Write}, path::Path};

use rust_static_site::{blocks::{extract_heading_content, get_heading_block_tag, markdown_to_blocks, markdown_to_html_node}, errors::NodeError};

fn clean_and_copy(origin: &Path, dest: &Path) -> Result<(), std::io::Error> {
    if dest.exists() {
        println!("Cleaning destination directory: {:?}", dest);
        std::fs::remove_dir_all(dest)?;
    }
    if origin.is_file() {
        println!("Copying file: {:?} -> {:?}", origin, dest);
        let dest_file = if dest.is_dir() {
            let filename = origin.file_name()
                .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidInput, "Invalid filename"))?;
            dest.join(filename)
        } else {
            dest.to_path_buf()
        };

        std::fs::copy(origin, dest_file)?;
    } else {
        println!("Copying directory contents: {:?} -> {:?}", origin, dest);
        std::fs::create_dir_all(dest)?;
        for entry in std::fs::read_dir(origin)? {
            let entry = entry?;
            let child_path = entry.path();
            let new_dest_path = dest.join(entry.file_name());

            clean_and_copy(&child_path, &new_dest_path)?;
        }
    }
    Ok(())
}

fn extract_title(markdown: &str) -> Result<String, NodeError> {
    match get_heading_block_tag(markdown).as_str() {
        "h1" => Ok(extract_heading_content(markdown)),
        _ => Err(NodeError::ParseError("First block must be an h1 heading".to_string()))
    }   
}

fn generate_page(from_path: &Path, template_path: &Path, dest_path: &Path, base_path: &Path) -> Result<(), std::io::Error> {
    println!("Generating page from {:?} -> {:?} using {:?}", from_path, dest_path, template_path);
    let mut source_file = std::fs::File::open(from_path)?;
    let mut source_text = String::new();
    source_file.read_to_string(&mut source_text)?;

    let mut template_file = std::fs::File::open(template_path)?;
    let mut template_text = String::new();
    template_file.read_to_string(&mut template_text)?;


    let source_html = markdown_to_html_node(&source_text)
        .to_html()
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, format!("HTML Conversion failed: {:?}", e)))?;
    
    let page_title = extract_title(&markdown_to_blocks(&source_text).first().unwrap_or(&"".to_string()))
        .unwrap_or_default();
    println!("DEBUG: TITLE = {page_title}");

    let page_html = template_text
        .replace("{{ Title }}", &page_title)
        .replace("{{ Content }}", &source_html)
        .replace(r#"href="/"#, &format!(r#"href="{}"#, base_path.display()))
        .replace(r#"src="/"#, &format!(r#"src="{}"#, base_path.display()));

    if dest_path.is_file() {
        println!("{:?} file already exists.", dest_path);
    } else {
        println!("{:?} doesn't exist, creating it now...", dest_path);
    }

    let mut dest_file = std::fs::File::create(dest_path)?;
    dest_file.write_all(page_html.as_bytes())?;
    Ok(())
}

fn generate_page_recursive(dir_path_content: &Path, template_path: &Path, dest_dir_path: &Path, base_path: &Path) -> Result<(), std::io::Error> {
    println!("Recursively generating website...");
    for entry in std::fs::read_dir(dir_path_content)? {
        let entry = entry?;
        let child_path = entry.path();
        println!("Examining path: {:?}", child_path);
        let new_dest_path = dest_dir_path.join( entry.file_name() );
        if child_path.is_dir() {
            println!("Path: {:?}", child_path);
            println!("Parent Path: {:?}", child_path.parent());
            println!("Making Dest Path: {:?}", &new_dest_path);
            std::fs::create_dir(&new_dest_path)?;
            println!("Does it now exists? {}", &new_dest_path.exists());
            generate_page_recursive(&child_path, template_path, &new_dest_path, base_path)?;
        } else if child_path.is_file() && child_path.extension().map_or(false, |ext| ext == "md") {
            println!("Found markdown file: {:?}", child_path);
            let parent_path = new_dest_path.parent()
                .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidInput, "No parent directory"))?;
            let dest_filepath = parent_path.join(Path::new(
                & format!("{}.html",
                    &new_dest_path.file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("unnamed")
                )
            ));
            println!("Generating new file: {:?}", dest_filepath);
            generate_page(&child_path, template_path, &dest_filepath, base_path)?;
        }
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let source = Path::new("content");
    let dest = Path::new("docs");
    let basepathstr = std::env::args().skip(1).next().unwrap_or("/".to_string());
    let basepath = Path::new(&basepathstr);

    clean_and_copy(Path::new("static"), dest)?;
    generate_page_recursive(
        source, 
        Path::new("template.html"), 
        dest,
        basepath,
    )?;

    Ok(())
}

#[cfg(test)]
mod maintests {
    use super::*;

    #[test]
    fn title_extraction() {
        let markdown = "# Hello";
        let title = extract_title(markdown).unwrap();
        assert_eq!("Hello", title);
    }
}