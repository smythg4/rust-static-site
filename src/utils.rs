use crate::errors::NodeError;
use crate::textnode::{TextNode, TextType};
use regex::Regex;

pub fn extract_markdown_images(text: &str) -> Vec<(&str, &str)> {
    let re = Regex::new(r"!\[([^\[\]]*)\]\(([^\(\)]*)\)").unwrap();
    
    re.captures_iter(text)
        .map(|cap| {
            let (_, [alt_text, url]) = cap.extract();
            (alt_text, url)
        })
        .collect()
}

pub fn extract_markdown_links(text: &str) -> Vec<(&str, &str)> {
    let re = Regex::new(r"\[([^\[\]]*)\]\(([^\(\)]*)\)").unwrap();
    
    re.captures_iter(text)
        .filter(|cap| {
            let start_pos = cap.get(0).unwrap().start();
            start_pos == 0 || text.chars().nth(start_pos - 1) != Some('!')
        })
        .map(|cap| {
            let (_, [link_text, url]) = cap.extract();
            (link_text, url)
        })
        .collect()
}

pub fn split_nodes_delimeter(old_nodes: Vec<TextNode>, delimeter: &str, text_type: TextType) -> Result<Vec<TextNode>, NodeError> {
    let mut new_nodes = Vec::new();

    for node in old_nodes {
        if &node.text_type != &TextType::Plain {
            new_nodes.push(node);
            continue;
        }
        if !node.text.contains(delimeter) {
            new_nodes.push(node);
            continue;
        }

        let parts: Vec<&str> = node.text.split(delimeter).collect();
        if parts.len() %2 == 0 {
            return Err(NodeError::ParseError(node.text.clone().to_string()));
        }

        for (i, part) in parts.iter().enumerate() {
            if i % 2 == 0 {
                if !part.is_empty() {
                    new_nodes.push( TextNode {
                        text: part.to_string(),
                        text_type: TextType::Plain,
                        url: None,
                    });
                }
            } else {
                if !part.is_empty() {
                    new_nodes.push( TextNode {
                        text: part.to_string(),
                        text_type,
                        url: None,
                    });
                }

            }
        }
    }

    Ok(new_nodes)
}

pub fn split_nodes_link(old_nodes: Vec<TextNode>) -> Result<Vec<TextNode>, NodeError> {
    let mut new_nodes = Vec::new();

    for node in old_nodes {
        if node.text.is_empty() {
            continue;
        }
        if node.text_type != TextType::Plain {
            new_nodes.push(node);
            continue;
        }
        let links = extract_markdown_links(&node.text);
        println!("{:?}", links);
        if links.is_empty() {
            new_nodes.push(node);
            continue;
        }
        let (title, url) = links.get(0).unwrap();
        let pat = format!("[{}]({})", title, url);
        println!("Hunting for pattern: {}", pat);
        let sections: Vec<_> = node.text.splitn(2, pat.as_str()).collect();
        println!("Found sections: {:?}", sections);
        if let Some(new_text) = sections.get(0) {
            println!("Making new plain text node from: {}", new_text);
            let new_node = TextNode{ text: new_text.to_string(), text_type: TextType::Plain, url: None };
            new_nodes.push(new_node);
        }

        // make the link node
        let new_node = TextNode{ text: title.to_string(), text_type: TextType::Link, url: Some(url.to_string()) };
        new_nodes.push(new_node);

        // parse the remainder of the node if required
        if let Some(remaining_text) = sections.get(1) {
            let remaining_node = TextNode{ text: remaining_text.to_string(), text_type: TextType::Plain, url: None };
            let extra_nodes = split_nodes_link(vec![remaining_node])?;
            new_nodes.extend(extra_nodes);
        }
    }
    Ok(new_nodes)
}

pub fn split_nodes_image(old_nodes: Vec<TextNode>) -> Result<Vec<TextNode>, NodeError> {
    let mut new_nodes = Vec::new();

    for node in old_nodes {
        if node.text.is_empty() {
            continue;
        }
        if node.text_type != TextType::Plain {
            new_nodes.push(node);
            continue;
        }
        let images = extract_markdown_images(&node.text);
        if images.is_empty() {
            new_nodes.push(node);
            continue;
        }
        let (alt_text, url) = images.get(0).unwrap();
        let pat = format!("![{}]({})", alt_text, url);
        //println!("Hunting for pattern: {}", pat);
        let sections: Vec<_> = node.text.splitn(2, pat.as_str()).collect();
        //println!("Found sections: {:?}", sections);
        if let Some(new_text) = sections.get(0) {
            //println!("Making new plain text node from: {}", new_text);
            let new_node = TextNode{ text: new_text.to_string(), text_type: TextType::Plain, url: None };
            new_nodes.push(new_node);
        }

        // make the image node
        let new_node = TextNode{ text: alt_text.to_string(), text_type: TextType::Image, url: Some(url.to_string()) };
        new_nodes.push(new_node);

        // parse the remainder of the node if required
        if let Some(remaining_text) = sections.get(1) {
            let remaining_node = TextNode{ text: remaining_text.to_string(), text_type: TextType::Plain, url: None };
            let extra_nodes = split_nodes_image(vec![remaining_node])?;
            new_nodes.extend(extra_nodes);
        }
    }
    Ok(new_nodes)
}

pub fn text_to_textnodes(text: &str) -> Result<Vec<TextNode>, NodeError> {
    let inital_node = TextNode {
        text: text.to_string(),
        text_type: TextType::Plain,
        url: None,
    };
    let mut final_nodes = vec![inital_node];

    final_nodes = split_nodes_delimeter(final_nodes, "_", TextType::Italic)?;
    final_nodes = split_nodes_delimeter(final_nodes, "`", TextType::Code)?;
    final_nodes = split_nodes_delimeter(final_nodes, "**", TextType::Bold)?;
    final_nodes = split_nodes_image(final_nodes)?;
    final_nodes = split_nodes_link(final_nodes)?;

    Ok(final_nodes)
}