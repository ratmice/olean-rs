use std::io;
use crate::trie::{Trie, TrieCommon, iter::Values, TrieKey, LastAncestorIter};
use crate::loader::Loader;
use crate::types::{Modification, KToken};

static MAX_PREC: u32 = 1024;
static ARROW_PREC: u32 = 25;
static PLUS_PREC: u32 = 65;
static TOKENS: &[(&str, u32)] = &[
    ("fun", 0), ("Pi", 0), ("let", 0), ("in", 0), ("at", 0),
    ("have", 0), ("assume", 0), ("show", 0), ("suffices", 0),
    ("do", 0), ("if", 0), ("then", 0), ("else", 0), ("by", 0),
    ("hiding", 0), ("replacing", 0), ("renaming", 0),
    ("from", 0), ("(", MAX_PREC), ("`(", MAX_PREC), ("``(", MAX_PREC),
    ("```(", MAX_PREC), ("`[", MAX_PREC), ("`", MAX_PREC),
    ("%%", MAX_PREC), ("()", MAX_PREC), ("(::)", MAX_PREC), (")", 0), ("'", 0),
    ("{", MAX_PREC), ("}", 0), ("_", MAX_PREC),
    ("[", MAX_PREC), ("#[", MAX_PREC), ("]", 0), ("⦃", MAX_PREC), ("⦄", 0), (".(", 0),
    ("{!", MAX_PREC), ("!}", 0),
    ("Type", MAX_PREC), ("Type*", MAX_PREC), ("Sort", MAX_PREC), ("Sort*", MAX_PREC),
    ("(:", MAX_PREC), (":)", 0), (".(", MAX_PREC), ("._", MAX_PREC),
    ("⟨", MAX_PREC), ("⟩", 0), ("^", 0),
    ("//", 0), ("|", 0), ("with", 0), ("without", 0), ("..", 0), ("...", 0), (",", 0),
    (".", 0), (":", 0), ("!", 0), ("calc", 0), (":=", 0), ("--", 0), ("#", MAX_PREC),
    ("/-", 0), ("/--", 0), ("/-!", 0), ("begin", MAX_PREC), ("using", 0),
    ("@@", MAX_PREC), ("@", MAX_PREC),
    ("sorry", MAX_PREC), ("+", PLUS_PREC), ("->", ARROW_PREC), ("<-", 0),
    ("match", 0), ("^.", MAX_PREC+1),
    ("renaming", 0), ("extends", 0)];

static COMMANDS: &[&str] = &[
    "theorem", "axiom", "axioms", "variable", "protected", "private", "hide",
    "definition", "meta", "mutual", "example", "noncomputable", "abbreviation",
    "variables", "parameter", "parameters", "constant", "constants",
    "using_well_founded", "[whnf]",
    "end", "namespace", "section", "prelude",
    "import", "inductive", "coinductive", "structure", "class", "universe", "universes", "local",
    "precedence", "reserve", "infixl", "infixr", "infix", "postfix", "prefix", "notation",
    "set_option", "open", "export", "@[",
    "attribute", "instance", "include", "omit", "init_quotient",
    "declare_trace", "add_key_equivalence",
    "run_cmd", "#check", "#reduce", "#eval", "#print", "#help", "#exit",
    "#compile", "#unify"];

static ALIASES: &[(&str, &str, Option<u32>)] = &[
    ("λ", "fun", Some(0)),
    ("forall", "Pi", Some(0)), ("∀", "Pi", Some(0)), ("Π", "Pi", Some(0)),
    ("(|", "⟨", Some(MAX_PREC)), ("|)", "⟩", Some(0)),
    ("→", "->", Some(ARROW_PREC)), ("←", "<-", Some(0)),
    ("lemma", "theorem", None), ("def", "definition", None)];

#[derive(Debug)] pub struct TokenTable(Trie<KToken>);

impl TokenTable {
    pub fn new() -> TokenTable {
        let mut table = TokenTable(Trie::new());
        for (s, prec) in TOKENS {
            table.insert(KToken{tk: s.to_string(), prec: Some(*prec)}) }
        for s in COMMANDS {
            table.insert(KToken{tk: s.to_string(), prec: None}) }
        for (s1, s2, prec) in ALIASES {
            table.0.insert(s1, KToken{tk: s2.to_string(), prec: *prec}); }
        table
    }

    fn insert(&mut self, tk: KToken) {
        self.0.insert_nv(tk.tk.encode(), tk);
    }

    pub fn search(&self) -> TokenSearch { self.0.last_ancestor_iter() }

    pub fn load(&mut self, load: &mut Loader) -> io::Result<()> {
        for n in &load.order {
            let mods = Loader::get_mods(&mut load.map, n.clone())?;
            for m in mods {
                match m {
                    Modification::Token(tk) => self.insert(tk.clone()),
                    _ => ()
                }
            }
        }
        Ok(())
    }
}

impl<'a> IntoIterator for &'a TokenTable {
    type Item = &'a KToken;
    type IntoIter = Values<'a, KToken>;
    fn into_iter(self) -> Self::IntoIter { self.0.values() }
}

type TokenSearch<'a> = LastAncestorIter<'a, KToken>;
