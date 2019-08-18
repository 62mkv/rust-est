/*
_A_ omadussõna - algvõrre (adjektiiv - positiiv), nii käänduvad kui käändumatud, nt kallis või eht,
_C_ omadussõna - keskvõrre (adjektiiv - komparatiiv), nt laiem,
_U_ omadussõna - ülivõrre (adjektiiv - superlatiiv), nt pikim,
_S_ nimisõna (substantiiv), nt asi,
_N_ põhiarvsõna (kardinaalnumeraal), nt kaks,
_O_ järgarvsõna (ordinaalnumeraal), nt teine.
_P_ asesõna (pronoomen), nt mina, see

Tabelis 2 on esitatud ka järgmiste sõnaliikide sagedused:

_V_ tegusõna (verb), nt tegema
_D_ määrsõna (adverb), nii täistähenduslikud, pro- kui ka afiksaaladverbid, nt kiiresti, siis, üle (jääma)
_J_ sidesõna (konjunktsioon), nt ja, kui
_K_ kaassõna (pre/postpositsioon), nt maja all, üle tee
_Y_ lühend, nt USA
*/

use std::fmt::{Display, Formatter};
use std::str::FromStr;

use itertools::Itertools;

#[derive(Debug)]
pub struct DeclinationType {
    types: Vec<u8>
}

#[derive(Display)]
pub enum PartOfSpeech {
    Noun,
    Verb,
    Adjective,
    Adverb,
    Numeral,
    Pronoun,
    Conjunction,
    Interjection
}

impl FromStr for PartOfSpeech {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "adjg" => Ok(PartOfSpeech::Adjective), //geographic
            "adj" => Ok(PartOfSpeech::Adjective),
            "s" => Ok(PartOfSpeech::Noun),
            "prop" => Ok(PartOfSpeech::Noun), // proper name
            "adv" => Ok(PartOfSpeech::Adverb),
            "v" => Ok(PartOfSpeech::Verb),
            "interj" => Ok(PartOfSpeech::Interjection),
            "konj" => Ok(PartOfSpeech::Conjunction),
            "num" => Ok(PartOfSpeech::Numeral),
            "pron" => Ok(PartOfSpeech::Pronoun),
            _ => {
                let mut msg = String::from("Unknown part of speech identifier: ");
                msg.push_str(s);
                Err(msg)
            }
        }
    }
}

impl FromStr for DeclinationType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(num) = u8::from_str(s) {
            Ok(DeclinationType {
                types: vec![num]
            })
        } else {
            let vec = s.split("_&_")
                .map(|s| s.trim())
                .map(|s| s.trim_end_matches("?"))
                .map(|i| u8::from_str(i)
                    .map_err(|e| format!("error code while parsing value {}", i))
                    .unwrap())
                .collect();
            Ok(DeclinationType {
                types: vec
            })
        }
    }
}

impl Display for DeclinationType {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.types.iter().format(", "))
    }
}