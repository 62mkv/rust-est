use std::convert::From;
use std::fmt::Write;

use roxmltree::{Document, Node, NodeType};

fn find_children_by_tagname<'a>(node: &'a Node, path: Vec<&'a str>) -> Result<Vec<'a &Node>, String> {
    if path.len() > 0 {
        let mut res: Vec<&Node> = Vec::new();
        for child in node.children().
            filter(|n| n.tag_name().name().eq(path.get(0))) {
            if path.len() > 1 {
                if let Ok(&mut vec) = find_children_by_tagname(&child, path[1..].to_owned()) {
                    res.append(vec);
                }
            } else {
                res.push(&child);
            }
        }
        Ok(res)
    } else {
        Vec::new();
    }
}

fn parse_article(article: &Node) -> Result<String, String> {
    let mut result = String::new();
}

pub fn parse(input: &str) -> Result<String, String> {
    let doc = Document::parse(input).unwrap();
    let mut str = String::new();
    for node in doc.root_element().children()
        .filter(|n| n.node_type() == NodeType::Element) {
        let mut node_legend = parse_article(&node).unwrap();
        str.push_str(&node_legend);
        str.push(',');
    }

    Ok(str)
}
