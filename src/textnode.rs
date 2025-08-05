use crate::htmlnode::HtmlNode;
use std::collections::HashMap;

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum TextType {
    Plain,
    Bold,
    Italic,
    Code,
    Link,
    Image,
}

#[derive(PartialEq, Eq, Debug)]
pub struct TextNode {
    pub text: String,
    pub text_type: TextType,
    pub url: Option<String>,
}

impl From<TextNode> for HtmlNode {
    fn from(tnode: TextNode) -> HtmlNode {
        match tnode.text_type {
            TextType::Plain => HtmlNode::leaf_node(None, &tnode.text, None),
            TextType::Bold => HtmlNode::leaf_node(Some("b"), &tnode.text, None),
            TextType::Italic => HtmlNode::leaf_node(Some("i"), &tnode.text, None),
            TextType::Code => HtmlNode::leaf_node(Some("code"), &tnode.text, None),
            TextType::Link => {
                let mut props = HashMap::new();
                props.insert("href".to_string(), tnode.url.clone().unwrap());
                HtmlNode::leaf_node(Some("a"), &tnode.text, Some(props))
            },
            TextType::Image => {
                let mut props = HashMap::new();
                props.insert("src".to_string(), tnode.url.clone().unwrap());
                props.insert("alt".to_string(), tnode.text.clone());
                HtmlNode::leaf_node(Some("img"), "", Some(props))
            },
        }
    }
}