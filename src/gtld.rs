use gemini::gemtext::Builder;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Statement {
    pub subject: String,
    pub predicate: String,
    pub object: String,
}

pub struct Document {
    pub statements: Vec<Statement>,
}

impl Statement {
    pub fn new(s: String, p: String, o: String) -> Statement {
        Statement {
            subject: s,
            predicate: p,
            object: o,
        }
    }
}

/// A collection of RDF statements, order unimportant.
impl Document {
    pub fn new() -> Document {
        Document {
            statements: Vec::<Statement>::new(),
        }
    }

    pub fn add(&mut self, s: Statement) -> &Self {
        self.statements.push(s);
        self
    }

    pub fn len(&self) -> usize {
        self.statements.len()
    }

    /// Exports to a representation of the RDF in gemtext.
    pub fn to_gemtext(&mut self) -> gemini::Builder {
        let mut gemtext = Builder::new();
        for stmt in self.statements.iter() {
            let s = &stmt.subject;
            let p = &stmt.predicate;
            let o = &stmt.object;
            gemtext = gemtext
                .link(s, Some(s))
                .link(p, Some(p))
                .link(o, Some(o))
                .line();
        }
        gemtext
    }
}

use crate::namespace::{NS_FRBR, NS_RDF, NS_WIKIDATA};
#[test]
fn test_gemtext_object_property_assertion() {
    let stmt = Statement::new(
        format!("{NS_WIKIDATA}Q2026413"),
        format!("{NS_RDF}type"),
        format!("{NS_FRBR}Work"),
    );
    let mut rdf = Document::new();
    rdf.add(stmt.clone());
    assert_eq!(rdf.len(), 1);
    assert_eq!(rdf.statements[0], stmt);
}
