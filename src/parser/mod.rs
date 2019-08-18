use std::fmt::{Display, Error, Formatter, Write};
use std::time::Instant;

use roxmltree::{Document, Node, NodeType};

use evs::{DeclinationType, PartOfSpeech};

pub mod evs;

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

#[derive(Debug)]
struct DisplayOption<T>(pub Option<T>);

impl<T: Display> Display for DisplayOption<T> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self.0 {
            Some(ref v) => write!(f, "Some({})", v),
            None => write!(f, "None")
        }
    }
}

struct Article<'input> {
    lexeme: &'input str,
    part_of_speech: DisplayOption<PartOfSpeech>,
    declination_type: DisplayOption<DeclinationType>,
}

impl Display for Article<'_> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{} [part of speech = {}, decl. type = {}]", self.lexeme, self.part_of_speech, self.declination_type)
    }
}

fn parse_article<'a>(article: Node<'a, '_>) -> Result<Article<'a>, String> {
    let nodes = find_children_by_tagname(article, vec!["P", "mg"]).unwrap();
    let node = nodes.get(0).unwrap().to_owned();
    let body_nodes = find_children_by_tagname(node, vec!["m"]).unwrap();
    let part_of_speech_nodes = find_children_by_tagname(node, vec!["sl"]).unwrap();
    let declination_type_nodes = find_children_by_tagname(node, vec!["grg", "mt"]).unwrap();
    let result = Article {
        lexeme: body_nodes.get(0).unwrap().text().unwrap(),
        part_of_speech: DisplayOption(
            part_of_speech_nodes.get(0)
                .and_then(|n| n.text())
                .and_then(|t| Some(t.parse().unwrap()))
        ),
        declination_type: DisplayOption(
            declination_type_nodes.get(0)
                .and_then(|n| n.text())
                .and_then(|t| Some(t.parse().unwrap()))
        )
    };
    Ok(result)
}

pub fn parse(input: &str) -> Result<String, String> {
    let mut content = String::new();
    write!(content, "<root xmlns:x=\"https://www.w3schools.com/furniture\">{}</root>", input).unwrap();

    let start = Instant::now();
    let doc = Document::parse(&content).unwrap();
    let duration = start.elapsed();
    println!("XML parsing complete in {:?}", duration);

    let start = Instant::now();
    let mut str = String::new();
    for node in doc.root_element().children()
        .filter(|n| n.node_type() == NodeType::Element) {
        let art = parse_article(node).unwrap();
        if let None = art.part_of_speech.0 {
            println!("Parsed article: {}", art);
        }
    }

    Ok(format!("Processing finished in {:?}", start.elapsed()))
}
