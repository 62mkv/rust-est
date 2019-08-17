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

use std::fmt::Display;
use std::str::FromStr;

type DeclinationType = u8;

pub enum ComparisonDegree {
    Positive,
    Comparative,
    Superlative
}

#[derive(Display)]pub enum PartOfSpeech {
    Noun,
    Verb,
    Adjective,
    Adverb,
    Numeral,
    Pronoun,
    Conjunction,
    Preposition,
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
            _ => {
                let mut msg = String::from("Unknown part of speech identifier: ");
                msg.push_str(s);
                Err(msg)
            }
        }
    }
}