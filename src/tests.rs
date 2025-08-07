#[cfg(test)]
mod tests {
use std::collections::HashMap;
use crate::htmlnode::HtmlNode;
use crate::textnode::{TextNode, TextType};
use crate::utils::*;
use crate::blocks::*;

    #[test]
    fn test_props_to_html() {
        let ptest = HtmlNode {
            tag: None,
            value: None,
            children: Vec::new(),
            props: Some(HashMap::from([
                ("href".to_string(), "http://www.google.com".to_string()),
                ("target".to_string(), "_blank".to_string()),
            ])),
        };

        let result = ptest.props_to_html();
        println!("{}", result);
        assert_eq!(" href=\"http://www.google.com\" target=\"_blank\"", result);
    }

    #[test]
    fn test_to_html_with_children() {
        let child = HtmlNode::leaf_node(Some("span"), "child", None);
        let parent = HtmlNode::parent_node("div", vec![child], None);

        assert_eq!(parent.to_html().unwrap(), "<div><span>child</span></div>");
    }

    #[test]
    fn test_to_html_with_grandchildren() {
        let grandchild = HtmlNode::leaf_node(Some("b"), "grandchild", None);
        let child = HtmlNode::parent_node("span", vec![grandchild], None);
        let parent = HtmlNode::parent_node("div", vec![child], None);

        assert_eq!(parent.to_html().unwrap(), "<div><span><b>grandchild</b></span></div>");
    }

    #[test]
    fn text_text() {
        let node = TextNode { text_type: TextType::Plain, text: "This is a text node".to_string(), url: None };
        let html_node = HtmlNode::from(node);
        assert_eq!(html_node.tag, None);
        assert_eq!(html_node.value.unwrap(), "This is a text node");

        let node = TextNode { text_type: TextType::Bold, text: "This is bold text!".to_string(), url: None };
        let html_node = HtmlNode::from(node);
        assert_eq!(html_node.tag, Some("b".to_string()));
        assert_eq!(html_node.value.unwrap(), "This is bold text!");
    }

    #[test]
    fn test_split_delimter() {
        let node = TextNode {
            text: "This is a block with `code stuff` in it.".to_string(),
            text_type: TextType::Plain,
            url: None,
        };
        let new_nodes = split_nodes_delimeter(vec![node], "`", TextType::Code);

        let expect = vec![
            TextNode{ text: "This is a block with ".to_string(), text_type: TextType::Plain, url: None },
            TextNode{ text: "code stuff".to_string(), text_type: TextType::Code, url: None },
            TextNode{ text: " in it.".to_string(), text_type: TextType::Plain, url: None },
        ];
        assert_eq!(expect, new_nodes.unwrap());
    }

    #[test]
    fn test_split_images() {
        let node = TextNode{
            text: "This is text with an ![image](https://i.imgur.com/zjjcJKZ.png) and another ![second image](https://i.imgur.com/3elNhQu.png)".to_string(),
            text_type: TextType::Plain,
            url: None,
        };

        let new_nodes = split_nodes_image(vec![node]).unwrap();
        let expect = vec![
            TextNode{ text: "This is text with an ".to_string(), text_type: TextType::Plain, url: None },
            TextNode{ text: "image".to_string(), text_type: TextType::Image, url: Some("https://i.imgur.com/zjjcJKZ.png".to_string())},
            TextNode{ text: " and another ".to_string(), text_type: TextType::Plain, url: None},
            TextNode{ text: "second image".to_string(), text_type: TextType::Image, url: Some("https://i.imgur.com/3elNhQu.png".to_string())}
        ];
        assert_eq!(expect, new_nodes);
    }

    #[test]
    fn test_split_links() {
        let node = TextNode {
            text: "This is text with a link [to boot dev](https://www.boot.dev) and [to youtube](https://www.youtube.com/@bootdotdev)".to_string(),
            text_type: TextType::Plain,
            url: None,
        };

        let new_nodes = split_nodes_link(vec![node]).unwrap();
        let expect = vec![
            TextNode{ text: "This is text with a link ".to_string(), text_type: TextType::Plain, url: None },
            TextNode{ text: "to boot dev".to_string(), text_type: TextType::Link, url: Some("https://www.boot.dev".to_string())},
            TextNode{ text: " and ".to_string(), text_type: TextType::Plain, url: None},
            TextNode{ text: "to youtube".to_string(), text_type: TextType::Link, url: Some("https://www.youtube.com/@bootdotdev".to_string())},
        ];
        assert_eq!(expect, new_nodes);
    }

    #[test]
    fn markdown_to_block_test() {
            let markdown = r#"This is a **bolded** paragraph

This is another paragraph with _italic_ text and `code` here
This is the same paragraph on a new line

- This is a list
- with items"#;

    let blocks = markdown_to_blocks(markdown);
    println!("{:?}", blocks);
    let expected = vec![
        "This is a **bolded** paragraph",
        "This is another paragraph with _italic_ text and `code` here\nThis is the same paragraph on a new line",
        "- This is a list\n- with items",
    ];
    assert_eq!(expected, blocks);

    }

    #[test]
    fn test_paragraph() {
        let markdown = r"This is **bolded** paragraph
text in a p
tag here

This is another paragraph with _italic_ text and `code` here
";
        let node = markdown_to_html_node(markdown);
        let html = node.to_html().unwrap();
        let expected = "<div><p>This is <b>bolded</b> paragraph text in a p tag here</p><p>This is another paragraph with <i>italic</i> text and <code>code</code> here</p></div>";
        assert_eq!(expected, html);
    }

    #[test]
    fn test_codeblock() {
        let markdown = r"```
This is text that _should_ remain
the **same** even with inline stuff
```";
        let node = markdown_to_html_node(markdown);
        let html = node.to_html().unwrap();
        let expected = "<div><pre><code>This is text that _should_ remain\nthe **same** even with inline stuff\n</code></pre></div>";
        assert_eq!(expected, html);
    }
}