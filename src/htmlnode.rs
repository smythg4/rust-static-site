use std::collections::HashMap;
use crate::errors::NodeError;

#[derive(Debug)]
pub struct HtmlNode {
    pub tag: Option<String>,
    pub value: Option<String>,
    pub children: Vec<Box<HtmlNode>>,
    pub props: Option<HashMap<String, String>>,
}

impl HtmlNode {

    pub fn parent_node(tag: &str, children: Vec<HtmlNode>, props: Option<HashMap<String, String>>) -> Self {
        HtmlNode {
            tag: Some(tag.to_string()),
            value: None,
            children: children.into_iter().map(|c| Box::new(c)).collect(),
            props,
        }
    }

    pub fn leaf_node(tag: Option<&str>, value: &str, props: Option<HashMap<String, String>>) -> Self {
        let tag = match tag {
            None => None,
            Some(str) => Some(str.to_string())
        };
        HtmlNode {
            tag,
            value: Some(value.to_string()),
            children: Vec::new(),
            props,
        }
    }

    pub fn to_html(&self) -> Result<String, NodeError> {
        let mut finalhtml = String::new();
        // base case for leaf nodes
        if self.children.is_empty() {
            if self.value.is_none() {
                return Err(NodeError::ValueError(finalhtml));
            } else {
                let text = self.value.clone().unwrap();
                let props = self.props_to_html();
                match &self.tag {
                    Some(tag) => {
                        return Ok(format!("<{}{}>{}</{}>", tag, props, text, tag));
                    },
                    None => {
                        return Ok(format!("{}", text));
                    }
                }
            }
        }

        // now work on parent nodes
        if self.tag.is_none() {
            return Err(NodeError::ValueError(finalhtml));
        }
        let open_tag = format!("<{}>", &self.tag.clone().unwrap());
        let close_tag = format!("</{}>", &self.tag.clone().unwrap());
        finalhtml.push_str(&open_tag);
        for child in &self.children {
            let sub_html = child.to_html()?;
            finalhtml.push_str(&sub_html);
        }
        finalhtml.push_str(&close_tag);
        Ok(finalhtml)
    }

    pub fn props_to_html(&self) -> String {
        let mut result = String::new();
        if let Some(props) = &self.props {
            for (key, value) in props {
                result.push_str(format!(r#" {}="{}""#, key, value).as_str());
            }
        }

        result
    }
}