use std::collections::HashSet;
use std::fmt::{Display, Error, Formatter, Write};
use std::path::Path;
use std::time::Instant;

use csv;
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

#[derive(Debug)]
struct Article<'input> {
    lexeme: &'input str,
    part_of_speech: DisplayOption<Vec<PartOfSpeech>>,
    declination_type: DisplayOption<DeclinationType>,
    guid: &'input str,
}

impl Display for Article<'_> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{} {} [part of speech = {:?}, decl. type = {}]", self.lexeme, self.guid, self.part_of_speech, self.declination_type)
    }
}

fn get_first_node_body<'a>(nodes: Vec<Node<'a, '_>>) -> Option<&'a str> {
    nodes.get(0)
        .and_then(|n| n.text())
}

fn parse_article<'a>(article: Node<'a, '_>) -> Result<Article<'a>, String> {
    let nodes = find_children_by_tagname(article, vec!["P", "mg"]).unwrap();
    let node = nodes.get(0).unwrap().to_owned();
    let body_nodes = find_children_by_tagname(node, vec!["m"]).unwrap();
    let guid_nodes = find_children_by_tagname(article, vec!["G"]).unwrap();
    let mut part_of_speech_nodes = find_children_by_tagname(node, vec!["grg", "sl1"]).unwrap();
    part_of_speech_nodes.append(&mut find_children_by_tagname(node, vec!["sl"]).unwrap());

    let parts_of_speech_hs: HashSet<PartOfSpeech> = part_of_speech_nodes
        .iter()
        .map(|n| n.text())
        .map(|t| t.map(|t| t.parse().unwrap()))
        .filter(|o| o.is_some())
        .map(|o| o.unwrap())
        .collect();

    let mut parts_of_speech: Vec<PartOfSpeech> = Vec::new();
    parts_of_speech.extend(parts_of_speech_hs.into_iter());

    let declination_type_nodes = find_children_by_tagname(node, vec!["grg", "mt"]).unwrap();
    let result = Article {
        lexeme: body_nodes.get(0).unwrap().text().unwrap(),
        part_of_speech: DisplayOption(
            if parts_of_speech.len() > 0 {
                Some(parts_of_speech)
            } else {
                None
            }
        ),
        declination_type: DisplayOption(
            get_first_node_body(declination_type_nodes)
                .and_then(|t| Some(t.parse().unwrap()))
        ),
        guid: get_first_node_body(guid_nodes).expect("NO GUID detected"),
    };
    Ok(result)
}

pub fn parse(input: &str, folder: &str) -> Result<String, String> {
    let mut content = String::new();
    write!(content, "<root xmlns:x=\"https://www.w3schools.com/furniture\">{}</root>", input).unwrap();

    let start = Instant::now();
    let doc = Document::parse(&content).unwrap();
    let duration = start.elapsed();
    println!("XML parsing complete in {:?}", duration);

    let mut base_writer = csv::Writer::from_path(Path::new(folder).join("basic-form.csv")).unwrap();
    base_writer.write_record(&["Base word", "GUID"]);

    let mut parts_of_speech_writer = csv::Writer::from_path(Path::new(folder).join("parts_of_speech.csv")).unwrap();
    parts_of_speech_writer.write_record(&["GUID", "Part of speech"]);

    let start = Instant::now();
    for node in doc.root_element().children()
        .filter(|n| n.node_type() == NodeType::Element) {
        let art = parse_article(node).unwrap();

        base_writer.write_record(&[art.lexeme, art.guid]);

        if let None = art.part_of_speech.0 {
            println!("Parsed article: {}", art);
        }

        if let Some(parts_of_speech) = art.part_of_speech.0 {
            for p in parts_of_speech {
                let s = format!("{}", p);
                parts_of_speech_writer.write_record(&[art.guid, &s]);
            }
        }
    }

    base_writer.flush();
    parts_of_speech_writer.flush();
    Ok(format!("Processing finished in {:?}", start.elapsed()))
}
