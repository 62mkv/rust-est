use std::fmt::{Display, Error, Formatter};

use roxmltree::{Document, Node, NodeType};

fn find_children_by_tagname<'a, 'input: 'a>(node: Node<'a, 'input>, path: Vec<&'input str>) -> Result<Vec<Node<'a, 'input>>, String> {
    if path.len() > 0 {
        let mut res: Vec<Node> = Vec::new();
        for child in node.children()
            .filter(|n| (&n.tag_name().name()).eq(path.get(0).unwrap()))
            {
                if path.len() > 1 {
                    if let Ok(mut vec) = find_children_by_tagname(child, path[1..].to_owned()) {
                        res.append(vec.as_mut());
                    }
                } else {
                    res.push(child);
                }
            }
        Ok(res)
    } else {
        Ok(Vec::new())
    }
}

struct Article {
    lexeme: String,
    part_of_speech: String,
}

impl Display for Article {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{} [{}]", self.lexeme, self.part_of_speech)
    }
}

fn get_text_from_node(node: Option<&Node>) -> Option<String> {
    if let Some(n) = node {
        n.text().map(|s| s.to_string())
    } else {
        None
    }
}

fn parse_article(article: Node) -> Result<Article, String> {
    let nodes = find_children_by_tagname(article, vec!["P", "mg"]).unwrap();
    let node = nodes.get(0).unwrap().to_owned();
    let body_nodes = find_children_by_tagname(node, vec!["m"]).unwrap();
    let part_of_speech_nodes = find_children_by_tagname(node, vec!["sl"]).unwrap();
    let result = Article {
        lexeme: get_text_from_node(body_nodes.get(0)).unwrap_or("".to_string()),
        part_of_speech: get_text_from_node(part_of_speech_nodes.get(0)).unwrap_or("".to_string()),
    };
    Ok(result)
}

pub fn parse(input: &str) -> Result<String, String> {
    let doc = Document::parse(input).unwrap();
    let mut str = String::new();
    for node in doc.root_element().children()
        .filter(|n| n.node_type() == NodeType::Element) {
        let node_legend = parse_article(node).unwrap().to_string();
        str.push_str(&node_legend);
        str.push(',');
    }

    Ok(str)
}
