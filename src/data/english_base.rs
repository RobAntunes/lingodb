//! Base English linguistic data for standard Lingo database

use crate::core::*;

/// Standard English letters
pub const ENGLISH_LETTERS: &[(&str, &str)] = &[
    ("a", "vowel"),
    ("b", "consonant"), 
    ("c", "consonant"),
    ("d", "consonant"),
    ("e", "vowel"),
    ("f", "consonant"),
    ("g", "consonant"),
    ("h", "consonant"),
    ("i", "vowel"),
    ("j", "consonant"),
    ("k", "consonant"),
    ("l", "consonant"),
    ("m", "consonant"),
    ("n", "consonant"),
    ("o", "vowel"),
    ("p", "consonant"),
    ("q", "consonant"),
    ("r", "consonant"),
    ("s", "consonant"),
    ("t", "consonant"),
    ("u", "vowel"),
    ("v", "consonant"),
    ("w", "consonant"),
    ("x", "consonant"),
    ("y", "sometimes_vowel"),
    ("z", "consonant"),
];

/// IPA phonemes for English
pub const ENGLISH_PHONEMES: &[(&str, &str, &str)] = &[
    // Consonants
    ("/p/", "voiceless_bilabial_plosive", "pat"),
    ("/b/", "voiced_bilabial_plosive", "bat"),
    ("/t/", "voiceless_alveolar_plosive", "tap"),
    ("/d/", "voiced_alveolar_plosive", "dad"),
    ("/k/", "voiceless_velar_plosive", "cat"),
    ("/g/", "voiced_velar_plosive", "got"),
    ("/f/", "voiceless_labiodental_fricative", "fat"),
    ("/v/", "voiced_labiodental_fricative", "vat"),
    ("/θ/", "voiceless_dental_fricative", "think"),
    ("/ð/", "voiced_dental_fricative", "this"),
    ("/s/", "voiceless_alveolar_fricative", "sat"),
    ("/z/", "voiced_alveolar_fricative", "zap"),
    ("/ʃ/", "voiceless_postalveolar_fricative", "ship"),
    ("/ʒ/", "voiced_postalveolar_fricative", "measure"),
    ("/h/", "voiceless_glottal_fricative", "hat"),
    ("/tʃ/", "voiceless_postalveolar_affricate", "chat"),
    ("/dʒ/", "voiced_postalveolar_affricate", "judge"),
    ("/m/", "bilabial_nasal", "mat"),
    ("/n/", "alveolar_nasal", "nat"),
    ("/ŋ/", "velar_nasal", "sing"),
    ("/l/", "alveolar_lateral", "lat"),
    ("/r/", "alveolar_approximant", "rat"),
    ("/j/", "palatal_approximant", "yes"),
    ("/w/", "labial_velar_approximant", "wet"),
    
    // Vowels (monophthongs)
    ("/i/", "close_front_unrounded", "beat"),
    ("/ɪ/", "near_close_front_unrounded", "bit"),
    ("/e/", "close_mid_front_unrounded", "bait"),
    ("/ɛ/", "open_mid_front_unrounded", "bet"),
    ("/æ/", "near_open_front_unrounded", "bat"),
    ("/ɑ/", "open_back_unrounded", "father"),
    ("/ɔ/", "open_mid_back_rounded", "bought"),
    ("/o/", "close_mid_back_rounded", "boat"),
    ("/ʊ/", "near_close_back_rounded", "book"),
    ("/u/", "close_back_rounded", "boot"),
    ("/ʌ/", "open_mid_back_unrounded", "but"),
    ("/ə/", "schwa", "about"),
    ("/ɚ/", "r_colored_schwa", "butter"),
    
    // Diphthongs
    ("/aɪ/", "diphthong", "bite"),
    ("/aʊ/", "diphthong", "bout"),
    ("/ɔɪ/", "diphthong", "boy"),
    ("/eɪ/", "diphthong", "bait"),
    ("/oʊ/", "diphthong", "boat"),
];

/// Common English morphemes with etymology
#[derive(Clone, Copy)]
pub struct MorphemeData {
    pub morpheme: &'static str,
    pub morph_type: MorphemeType,
    pub meaning: &'static str,
    pub etymology: EtymologyOrigin,
    pub productivity: f32, // 0.0-1.0 how productive in forming new words
}

/// Common prefixes
pub const ENGLISH_PREFIXES: &[MorphemeData] = &[
    // Latin prefixes
    MorphemeData { morpheme: "pre", morph_type: MorphemeType::Prefix, meaning: "before", etymology: EtymologyOrigin::Latin, productivity: 0.9 },
    MorphemeData { morpheme: "post", morph_type: MorphemeType::Prefix, meaning: "after", etymology: EtymologyOrigin::Latin, productivity: 0.8 },
    MorphemeData { morpheme: "anti", morph_type: MorphemeType::Prefix, meaning: "against", etymology: EtymologyOrigin::Greek, productivity: 0.9 },
    MorphemeData { morpheme: "de", morph_type: MorphemeType::Prefix, meaning: "reverse/remove", etymology: EtymologyOrigin::Latin, productivity: 0.8 },
    MorphemeData { morpheme: "dis", morph_type: MorphemeType::Prefix, meaning: "not/opposite", etymology: EtymologyOrigin::Latin, productivity: 0.8 },
    MorphemeData { morpheme: "re", morph_type: MorphemeType::Prefix, meaning: "again", etymology: EtymologyOrigin::Latin, productivity: 0.9 },
    MorphemeData { morpheme: "sub", morph_type: MorphemeType::Prefix, meaning: "under", etymology: EtymologyOrigin::Latin, productivity: 0.7 },
    MorphemeData { morpheme: "super", morph_type: MorphemeType::Prefix, meaning: "above", etymology: EtymologyOrigin::Latin, productivity: 0.8 },
    MorphemeData { morpheme: "trans", morph_type: MorphemeType::Prefix, meaning: "across", etymology: EtymologyOrigin::Latin, productivity: 0.7 },
    MorphemeData { morpheme: "inter", morph_type: MorphemeType::Prefix, meaning: "between", etymology: EtymologyOrigin::Latin, productivity: 0.8 },
    MorphemeData { morpheme: "non", morph_type: MorphemeType::Prefix, meaning: "not", etymology: EtymologyOrigin::Latin, productivity: 0.8 },
    MorphemeData { morpheme: "ex", morph_type: MorphemeType::Prefix, meaning: "out/former", etymology: EtymologyOrigin::Latin, productivity: 0.7 },
    
    // Greek prefixes
    MorphemeData { morpheme: "bio", morph_type: MorphemeType::Prefix, meaning: "life", etymology: EtymologyOrigin::Greek, productivity: 0.9 },
    MorphemeData { morpheme: "geo", morph_type: MorphemeType::Prefix, meaning: "earth", etymology: EtymologyOrigin::Greek, productivity: 0.8 },
    MorphemeData { morpheme: "tele", morph_type: MorphemeType::Prefix, meaning: "far", etymology: EtymologyOrigin::Greek, productivity: 0.8 },
    MorphemeData { morpheme: "micro", morph_type: MorphemeType::Prefix, meaning: "small", etymology: EtymologyOrigin::Greek, productivity: 0.9 },
    MorphemeData { morpheme: "macro", morph_type: MorphemeType::Prefix, meaning: "large", etymology: EtymologyOrigin::Greek, productivity: 0.7 },
    MorphemeData { morpheme: "mega", morph_type: MorphemeType::Prefix, meaning: "great", etymology: EtymologyOrigin::Greek, productivity: 0.8 },
    MorphemeData { morpheme: "hyper", morph_type: MorphemeType::Prefix, meaning: "over", etymology: EtymologyOrigin::Greek, productivity: 0.8 },
    MorphemeData { morpheme: "hypo", morph_type: MorphemeType::Prefix, meaning: "under", etymology: EtymologyOrigin::Greek, productivity: 0.6 },
    MorphemeData { morpheme: "meta", morph_type: MorphemeType::Prefix, meaning: "beyond/change", etymology: EtymologyOrigin::Greek, productivity: 0.8 },
    MorphemeData { morpheme: "auto", morph_type: MorphemeType::Prefix, meaning: "self", etymology: EtymologyOrigin::Greek, productivity: 0.8 },
    
    // Germanic prefixes
    MorphemeData { morpheme: "over", morph_type: MorphemeType::Prefix, meaning: "above/too much", etymology: EtymologyOrigin::Germanic, productivity: 0.8 },
    MorphemeData { morpheme: "under", morph_type: MorphemeType::Prefix, meaning: "below/too little", etymology: EtymologyOrigin::Germanic, productivity: 0.8 },
    MorphemeData { morpheme: "un", morph_type: MorphemeType::Prefix, meaning: "not", etymology: EtymologyOrigin::Germanic, productivity: 0.9 },
    MorphemeData { morpheme: "fore", morph_type: MorphemeType::Prefix, meaning: "before", etymology: EtymologyOrigin::Germanic, productivity: 0.6 },
    MorphemeData { morpheme: "out", morph_type: MorphemeType::Prefix, meaning: "surpass", etymology: EtymologyOrigin::Germanic, productivity: 0.7 },
];

/// Common suffixes
pub const ENGLISH_SUFFIXES: &[MorphemeData] = &[
    // Noun-forming suffixes
    MorphemeData { morpheme: "tion", morph_type: MorphemeType::Suffix, meaning: "action/result", etymology: EtymologyOrigin::Latin, productivity: 0.9 },
    MorphemeData { morpheme: "ity", morph_type: MorphemeType::Suffix, meaning: "quality/state", etymology: EtymologyOrigin::Latin, productivity: 0.8 },
    MorphemeData { morpheme: "ment", morph_type: MorphemeType::Suffix, meaning: "action/result", etymology: EtymologyOrigin::Latin, productivity: 0.8 },
    MorphemeData { morpheme: "ness", morph_type: MorphemeType::Suffix, meaning: "quality/state", etymology: EtymologyOrigin::Germanic, productivity: 0.9 },
    MorphemeData { morpheme: "er", morph_type: MorphemeType::Suffix, meaning: "agent/comparative", etymology: EtymologyOrigin::Germanic, productivity: 0.9 },
    MorphemeData { morpheme: "or", morph_type: MorphemeType::Suffix, meaning: "agent", etymology: EtymologyOrigin::Latin, productivity: 0.7 },
    MorphemeData { morpheme: "ist", morph_type: MorphemeType::Suffix, meaning: "practitioner", etymology: EtymologyOrigin::Greek, productivity: 0.8 },
    MorphemeData { morpheme: "ism", morph_type: MorphemeType::Suffix, meaning: "doctrine/belief", etymology: EtymologyOrigin::Greek, productivity: 0.8 },
    MorphemeData { morpheme: "ship", morph_type: MorphemeType::Suffix, meaning: "state/skill", etymology: EtymologyOrigin::Germanic, productivity: 0.7 },
    MorphemeData { morpheme: "hood", morph_type: MorphemeType::Suffix, meaning: "state/group", etymology: EtymologyOrigin::Germanic, productivity: 0.6 },
    
    // Adjective-forming suffixes
    MorphemeData { morpheme: "able", morph_type: MorphemeType::Suffix, meaning: "capable of", etymology: EtymologyOrigin::Latin, productivity: 0.9 },
    MorphemeData { morpheme: "ible", morph_type: MorphemeType::Suffix, meaning: "capable of", etymology: EtymologyOrigin::Latin, productivity: 0.7 },
    MorphemeData { morpheme: "ful", morph_type: MorphemeType::Suffix, meaning: "full of", etymology: EtymologyOrigin::Germanic, productivity: 0.8 },
    MorphemeData { morpheme: "less", morph_type: MorphemeType::Suffix, meaning: "without", etymology: EtymologyOrigin::Germanic, productivity: 0.8 },
    MorphemeData { morpheme: "ous", morph_type: MorphemeType::Suffix, meaning: "having quality", etymology: EtymologyOrigin::Latin, productivity: 0.7 },
    MorphemeData { morpheme: "ic", morph_type: MorphemeType::Suffix, meaning: "pertaining to", etymology: EtymologyOrigin::Greek, productivity: 0.8 },
    MorphemeData { morpheme: "al", morph_type: MorphemeType::Suffix, meaning: "pertaining to", etymology: EtymologyOrigin::Latin, productivity: 0.8 },
    MorphemeData { morpheme: "ive", morph_type: MorphemeType::Suffix, meaning: "tending to", etymology: EtymologyOrigin::Latin, productivity: 0.8 },
    
    // Verb-forming suffixes
    MorphemeData { morpheme: "ize", morph_type: MorphemeType::Suffix, meaning: "make/become", etymology: EtymologyOrigin::Greek, productivity: 0.9 },
    MorphemeData { morpheme: "ify", morph_type: MorphemeType::Suffix, meaning: "make/become", etymology: EtymologyOrigin::Latin, productivity: 0.8 },
    MorphemeData { morpheme: "ate", morph_type: MorphemeType::Suffix, meaning: "make/become", etymology: EtymologyOrigin::Latin, productivity: 0.7 },
    
    // Technical suffixes
    MorphemeData { morpheme: "logy", morph_type: MorphemeType::Suffix, meaning: "study of", etymology: EtymologyOrigin::Greek, productivity: 0.8 },
    MorphemeData { morpheme: "graphy", morph_type: MorphemeType::Suffix, meaning: "writing/recording", etymology: EtymologyOrigin::Greek, productivity: 0.7 },
    MorphemeData { morpheme: "metry", morph_type: MorphemeType::Suffix, meaning: "measurement", etymology: EtymologyOrigin::Greek, productivity: 0.7 },
];

/// Common roots
pub const ENGLISH_ROOTS: &[MorphemeData] = &[
    // Greek roots
    MorphemeData { morpheme: "tech", morph_type: MorphemeType::Root, meaning: "art/skill", etymology: EtymologyOrigin::Greek, productivity: 0.8 },
    MorphemeData { morpheme: "log", morph_type: MorphemeType::Root, meaning: "word/study", etymology: EtymologyOrigin::Greek, productivity: 0.9 },
    MorphemeData { morpheme: "graph", morph_type: MorphemeType::Root, meaning: "write", etymology: EtymologyOrigin::Greek, productivity: 0.8 },
    MorphemeData { morpheme: "phon", morph_type: MorphemeType::Root, meaning: "sound", etymology: EtymologyOrigin::Greek, productivity: 0.7 },
    MorphemeData { morpheme: "photo", morph_type: MorphemeType::Root, meaning: "light", etymology: EtymologyOrigin::Greek, productivity: 0.8 },
    MorphemeData { morpheme: "chron", morph_type: MorphemeType::Root, meaning: "time", etymology: EtymologyOrigin::Greek, productivity: 0.7 },
    MorphemeData { morpheme: "meter", morph_type: MorphemeType::Root, meaning: "measure", etymology: EtymologyOrigin::Greek, productivity: 0.8 },
    MorphemeData { morpheme: "scope", morph_type: MorphemeType::Root, meaning: "view", etymology: EtymologyOrigin::Greek, productivity: 0.8 },
    MorphemeData { morpheme: "therm", morph_type: MorphemeType::Root, meaning: "heat", etymology: EtymologyOrigin::Greek, productivity: 0.7 },
    MorphemeData { morpheme: "hydr", morph_type: MorphemeType::Root, meaning: "water", etymology: EtymologyOrigin::Greek, productivity: 0.7 },
    MorphemeData { morpheme: "psych", morph_type: MorphemeType::Root, meaning: "mind", etymology: EtymologyOrigin::Greek, productivity: 0.8 },
    MorphemeData { morpheme: "path", morph_type: MorphemeType::Root, meaning: "feeling/disease", etymology: EtymologyOrigin::Greek, productivity: 0.8 },
    
    // Latin roots
    MorphemeData { morpheme: "port", morph_type: MorphemeType::Root, meaning: "carry", etymology: EtymologyOrigin::Latin, productivity: 0.7 },
    MorphemeData { morpheme: "ject", morph_type: MorphemeType::Root, meaning: "throw", etymology: EtymologyOrigin::Latin, productivity: 0.7 },
    MorphemeData { morpheme: "dict", morph_type: MorphemeType::Root, meaning: "speak", etymology: EtymologyOrigin::Latin, productivity: 0.7 },
    MorphemeData { morpheme: "duct", morph_type: MorphemeType::Root, meaning: "lead", etymology: EtymologyOrigin::Latin, productivity: 0.7 },
    MorphemeData { morpheme: "scrib", morph_type: MorphemeType::Root, meaning: "write", etymology: EtymologyOrigin::Latin, productivity: 0.7 },
    MorphemeData { morpheme: "script", morph_type: MorphemeType::Root, meaning: "write", etymology: EtymologyOrigin::Latin, productivity: 0.8 },
    MorphemeData { morpheme: "spec", morph_type: MorphemeType::Root, meaning: "look", etymology: EtymologyOrigin::Latin, productivity: 0.7 },
    MorphemeData { morpheme: "struct", morph_type: MorphemeType::Root, meaning: "build", etymology: EtymologyOrigin::Latin, productivity: 0.8 },
    MorphemeData { morpheme: "vert", morph_type: MorphemeType::Root, meaning: "turn", etymology: EtymologyOrigin::Latin, productivity: 0.7 },
    MorphemeData { morpheme: "vid", morph_type: MorphemeType::Root, meaning: "see", etymology: EtymologyOrigin::Latin, productivity: 0.7 },
    MorphemeData { morpheme: "voc", morph_type: MorphemeType::Root, meaning: "call/voice", etymology: EtymologyOrigin::Latin, productivity: 0.7 },
    MorphemeData { morpheme: "duc", morph_type: MorphemeType::Root, meaning: "lead", etymology: EtymologyOrigin::Latin, productivity: 0.7 },
];

/// Essential additional morphemes for better coverage
pub const ESSENTIAL_ADDITIONS: &[MorphemeData] = &[
    // Critical missing prefixes
    MorphemeData { morpheme: "co", morph_type: MorphemeType::Prefix, meaning: "together/with", etymology: EtymologyOrigin::Latin, productivity: 0.9 },
    MorphemeData { morpheme: "counter", morph_type: MorphemeType::Prefix, meaning: "against/opposite", etymology: EtymologyOrigin::Latin, productivity: 0.8 },
    MorphemeData { morpheme: "mis", morph_type: MorphemeType::Prefix, meaning: "wrongly/badly", etymology: EtymologyOrigin::Germanic, productivity: 0.9 },
    MorphemeData { morpheme: "extra", morph_type: MorphemeType::Prefix, meaning: "beyond/outside", etymology: EtymologyOrigin::Latin, productivity: 0.7 },
    MorphemeData { morpheme: "ultra", morph_type: MorphemeType::Prefix, meaning: "beyond/extreme", etymology: EtymologyOrigin::Latin, productivity: 0.7 },
    
    // Critical missing suffixes
    MorphemeData { morpheme: "ance", morph_type: MorphemeType::Suffix, meaning: "state/quality", etymology: EtymologyOrigin::Latin, productivity: 0.8 },
    MorphemeData { morpheme: "ence", morph_type: MorphemeType::Suffix, meaning: "state/quality", etymology: EtymologyOrigin::Latin, productivity: 0.8 },
    MorphemeData { morpheme: "ary", morph_type: MorphemeType::Suffix, meaning: "relating to", etymology: EtymologyOrigin::Latin, productivity: 0.7 },
    MorphemeData { morpheme: "ics", morph_type: MorphemeType::Suffix, meaning: "study/system", etymology: EtymologyOrigin::Greek, productivity: 0.7 },
    MorphemeData { morpheme: "age", morph_type: MorphemeType::Suffix, meaning: "action/result", etymology: EtymologyOrigin::French, productivity: 0.7 },
    
    // Critical missing roots
    MorphemeData { morpheme: "gen", morph_type: MorphemeType::Root, meaning: "birth/origin", etymology: EtymologyOrigin::Greek, productivity: 0.8 },
    MorphemeData { morpheme: "cap", morph_type: MorphemeType::Root, meaning: "take/seize", etymology: EtymologyOrigin::Latin, productivity: 0.7 },
    MorphemeData { morpheme: "cept", morph_type: MorphemeType::Root, meaning: "take/receive", etymology: EtymologyOrigin::Latin, productivity: 0.7 },
    MorphemeData { morpheme: "man", morph_type: MorphemeType::Root, meaning: "hand", etymology: EtymologyOrigin::Latin, productivity: 0.7 },
    MorphemeData { morpheme: "manu", morph_type: MorphemeType::Root, meaning: "hand", etymology: EtymologyOrigin::Latin, productivity: 0.7 },
    MorphemeData { morpheme: "mit", morph_type: MorphemeType::Root, meaning: "send", etymology: EtymologyOrigin::Latin, productivity: 0.7 },
    MorphemeData { morpheme: "pos", morph_type: MorphemeType::Root, meaning: "place/put", etymology: EtymologyOrigin::Latin, productivity: 0.8 },
    MorphemeData { morpheme: "sen", morph_type: MorphemeType::Root, meaning: "feel/sense", etymology: EtymologyOrigin::Latin, productivity: 0.7 },
    MorphemeData { morpheme: "sent", morph_type: MorphemeType::Root, meaning: "feel/sense", etymology: EtymologyOrigin::Latin, productivity: 0.7 },
    MorphemeData { morpheme: "ten", morph_type: MorphemeType::Root, meaning: "hold", etymology: EtymologyOrigin::Latin, productivity: 0.7 },
    MorphemeData { morpheme: "ven", morph_type: MorphemeType::Root, meaning: "come", etymology: EtymologyOrigin::Latin, productivity: 0.7 },
    MorphemeData { morpheme: "vent", morph_type: MorphemeType::Root, meaning: "come", etymology: EtymologyOrigin::Latin, productivity: 0.7 },
    MorphemeData { morpheme: "mov", morph_type: MorphemeType::Root, meaning: "move", etymology: EtymologyOrigin::Latin, productivity: 0.7 },
    MorphemeData { morpheme: "mot", morph_type: MorphemeType::Root, meaning: "move", etymology: EtymologyOrigin::Latin, productivity: 0.7 },
    MorphemeData { morpheme: "cred", morph_type: MorphemeType::Root, meaning: "believe", etymology: EtymologyOrigin::Latin, productivity: 0.7 },
];

/// Get all base data for English
pub fn get_english_base_data() -> (Vec<&'static str>, Vec<&'static str>, Vec<MorphemeData>) {
    let letters: Vec<&str> = ENGLISH_LETTERS.iter().map(|(l, _)| *l).collect();
    let phonemes: Vec<&str> = ENGLISH_PHONEMES.iter().map(|(p, _, _)| *p).collect();
    
    let mut morphemes = Vec::new();
    morphemes.extend_from_slice(ENGLISH_PREFIXES);
    morphemes.extend_from_slice(ENGLISH_SUFFIXES);
    morphemes.extend_from_slice(ENGLISH_ROOTS);
    
    (letters, phonemes, morphemes)
}