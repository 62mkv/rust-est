use std::collections::HashSet;
use std::convert::{TryFrom, TryInto};
use std::fmt::{Display, Error, Formatter, Write};
use std::path::Path;
use std::time::Instant;

use csv;
use regex::Regex;
use roxmltree::{Document, Node, NodeType};

use evs::{DeclinationType, PartOfSpeech};

use crate::parser::evs::PartOfSpeech::Verb;

use super::encoding;
use super::synthesis;

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

fn exploit_brackets(lexeme: &str) -> Vec<String> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(.*?)\[(.+)\](.*?)$").unwrap();
    }
    if let Some(caps) = RE.captures(lexeme) {
        let first_part = caps.get(1).unwrap().as_str();
        let middle = caps.get(2).unwrap().as_str();
        let last_part = caps.get(3).unwrap().as_str();
        let cut_brackets = String::from(first_part) + last_part;
        let uncut_brackets = String::from(first_part) + middle + last_part;
        vec![cut_brackets, uncut_brackets]
    } else {
        vec![lexeme.to_string()]
    }
}

fn get_baseforms_per_lexeme(lexeme: &str) -> Vec<String> {
    // remove '+' signs that should not be present in a base representation
    let word_components: Vec<&str> = lexeme.split('+').collect();
    let unplussed_lexeme = word_components.join("");
    lazy_static! {
        static ref RE: Regex = Regex::new(r"&em[al];").unwrap();
    }

    let unemaemled = RE.replace_all(&unplussed_lexeme, "");
    exploit_brackets(&unemaemled)
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

    let mut fmsynth_writer = csv::Writer::from_path(Path::new(folder).join("fmsynth.csv")).unwrap();
    fmsynth_writer.write_record(&["GUID", "Part of speech", "Declination type",
        "Options count", "Parallel forms count",
        "Form code", "Form representation", "Stem length"]);

    let mut hs: HashSet<Vec<PartOfSpeech>> = HashSet::new();

    let start = Instant::now();
    for node in doc.root_element().children()
        .filter(|n| n.node_type() == NodeType::Element) {
        let art = parse_article(node).unwrap();

        for ref baseform in get_baseforms_per_lexeme(art.lexeme) {
            base_writer.write_record(&[baseform, art.guid]);
        }

        if let Ok(mut lemma) = encoding::encode(&art.lexeme) {
            let (buffer, count) = synthesis::synthesize_encoded_vec(lemma);

            for &synthesis::SynthFormSet {
                declination_type,
                part_of_speech,
                number_of_options,
                parallel_forms,
                form_code,
                forms,
            } in &buffer[..count] {
                let part_of_speech = encoding::decode(&part_of_speech)?;
                let form_code = encoding::decode(&form_code)?;
                for &synthesis::SynthForm {
                    form,
                    stem_length
                } in &forms[..usize::try_from(parallel_forms).expect("Overflow")] {
                    if stem_length > 0 {
                        let form_string = encoding::decode(&form)?;
                        fmsynth_writer.write_record(&[art.guid, &part_of_speech, &declination_type.to_string(),
                            &number_of_options.to_string(), &parallel_forms.to_string(), &form_code, &form_string, &stem_length.to_string()]);
                    }
                }
            }
        }

        if let Some(ref parts_of_speech) = art.part_of_speech.0 {
            if parts_of_speech.len() > 1 {
                println!("Article with >1 parts of speech: {}", &art);
            }
        }

        if let Some(mut parts_of_speech) = art.part_of_speech.0 {
            for p in parts_of_speech.iter() {
                let s = format!("{}", p);
                parts_of_speech_writer.write_record(&[art.guid, &s]);
            }

            // sort part of speech
            parts_of_speech.sort();
            hs.insert(parts_of_speech);
        }
    }

    for x in hs.iter() {
        println!("{:?}", x);
    }

    base_writer.flush();
    parts_of_speech_writer.flush();
    Ok(format!("Processing finished in {:?}", start.elapsed()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unbracketed() {
        assert_eq!(get_baseforms_per_lexeme(&"õhuke[ne]"), vec!["õhuke", "õhukene"]);
        assert_eq!(get_baseforms_per_lexeme(&"aabits+"), vec!["aabits"]);
        assert_eq!(get_baseforms_per_lexeme(&"aabits+la[ne]"), vec!["aabitsla", "aabitslane"]);
        assert_eq!(get_baseforms_per_lexeme("&ema;roll-on&eml;-deodorant"), vec!["roll-on-deodorant"]);
    }
}