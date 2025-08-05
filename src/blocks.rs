use crate::htmlnode::HtmlNode;
use crate::utils::*;

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum BlockType {
    Paragraph,
    Heading,
    Code,
    Quote,
    UnorderedList,
    OrderedList,
}

pub fn is_quote(block: &str) -> bool {
    block.lines().into_iter()
        .all(|l| l.starts_with(">"))
}

pub fn is_ul(block: &str) -> bool {
    block.lines().into_iter()
        .all(|l| l.starts_with("- "))
}

pub fn is_ol(block: &str) -> bool {
    block.lines().enumerate().into_iter()
        .all(|(i, line)| {
            let pat = format!("{}. ", i+1);
            line.starts_with(&pat)
        })
}

pub fn is_heading(block: &str) -> bool {
    if !block.starts_with("#") {
        return false;
    }

    let hash_count = block.chars().take_while(|&c| c == '#').count();
    if hash_count == 0 || hash_count > 6 {
        return false;
    }

    block.chars().nth(hash_count) == Some(' ') || block.len() == hash_count
}

pub fn is_code(block: &str) -> bool {
    let mut lines = block.split("\n");
    let line_count = lines.clone().count();
    if line_count < 2 {
        return false;
    }

    if let Some(first_line) = lines.next() {
        if let Some(last_line) = lines.last() {
            return first_line == "```" && last_line == "```";
        }
    }
    return false;
}

pub fn block_to_blocktype(block: &str) -> BlockType {
    if is_heading(block) {
        return BlockType::Heading;
    }
    if is_code(block) {
        return BlockType::Code;
    }
    if is_quote(block) {
        return BlockType::Quote;
    }
    if is_ol(block) {
        return BlockType::OrderedList;
    }
    if is_ul(block) {
        return BlockType::UnorderedList;
    }

    BlockType::Paragraph
}

pub fn markdown_to_blocks(markdown: &str) -> Vec<String> {
    markdown.split("\n\n")
        .filter(|s| !s.is_empty())
        .map(|s| s.trim_end().to_string())
        .collect()
}

pub fn extract_code_content(block: &str) -> String {
    //block.split("\n").filter(|l| !l.starts_with("```")).collect::<Vec<_>>().join(" ")
    block.trim_start_matches("```\n")
        .trim_end_matches("```")
        .to_string()
}

pub fn extract_heading_content(block: &str) -> String {
    block.trim_start_matches("#")
        .trim_start()
        .to_string()
}

pub fn extract_quote_content(block: &str) -> String {
    println!("Extracting quote content from {:?}...", block);
    block.lines().into_iter()
        .filter(|line| !line.is_empty())
        .map(|line| line.trim_start_matches(">"))
        .map(|line| line.trim_start_matches(" "))
        .collect::<Vec<_>>()
        .join(" ")
}

pub fn extract_ul_nodes(block: &str) -> Vec<HtmlNode> {
    block.lines().into_iter()
        .map(|line| line.trim_start_matches("- "))
        .map(|line| text_to_children(line))
        .map(|child_nodes| HtmlNode::parent_node("li", child_nodes, None))
        .collect()
}

pub fn extract_ol_nodes(block: &str) -> Vec<HtmlNode> {
    block.lines().enumerate().into_iter()
        .map(|(i, line)| {
            let pat = format!("{}. ", i+1);
            line.trim_start_matches(&pat)
        })
        .map(|line| text_to_children(line))
        .map(|child_nodes| HtmlNode::parent_node("li", child_nodes, None))
        .collect()
}

pub fn text_to_children(block: &str) -> Vec<HtmlNode> {
    let text_nodes = text_to_textnodes(block).unwrap();
    let mut html_nodes = Vec::new();
    for tnode in text_nodes {
        html_nodes.push( HtmlNode::from(tnode) );
    }
    html_nodes
}

pub fn get_heading_block_tag(block: &str) -> String {
    println!("Determing heading block tag from: {}", block);
    let num = block.chars()
        .filter(|c| *c == '#')
        .count()
        .min(6);
    format!("h{num}")
}

pub fn markdown_to_html_node(markdown: &str) -> HtmlNode {
    let blocks = markdown_to_blocks(markdown);
    let mut nodes = Vec::new();

    for block in blocks {
        let block_type = block_to_blocktype(&block);
        match block_type {
            BlockType::Paragraph => {
                let clean_block = block.replace("\n", " ");
                let child_nodes = text_to_children(&clean_block);
                let this_node = HtmlNode::parent_node("p", child_nodes, None);
                nodes.push(this_node);
            },
            BlockType::Heading => {
                let tag = get_heading_block_tag(&block);
                let child_nodes = text_to_children(
                  extract_heading_content(&block).as_str()  
                );
                let this_node = HtmlNode::parent_node(tag.as_str(), child_nodes, None);
                nodes.push(this_node);
            },
            BlockType::Code => {
                let content = extract_code_content(&block);
                let code_node = HtmlNode::leaf_node(Some("code"), &content, None);
                let pre_node = HtmlNode::parent_node("pre", vec![code_node], None);
                nodes.push(pre_node);
            },
            BlockType::Quote => {
                let quote_content = extract_quote_content(&block);
                let child_nodes = text_to_children(&quote_content);
                let this_node = HtmlNode::parent_node("blockquote", child_nodes, None);
                nodes.push(this_node);
            },
            BlockType::UnorderedList => {
                let li_nodes = extract_ul_nodes(&block);
                let ul_node = HtmlNode::parent_node("ul", li_nodes, None);
                nodes.push(ul_node);
            },
            BlockType::OrderedList => {
                let li_nodes = extract_ol_nodes(&block);
                let ol_node = HtmlNode::parent_node("ol", li_nodes, None);
                nodes.push(ol_node);
            },
        }
    }
    HtmlNode::parent_node("div", nodes, None)
}