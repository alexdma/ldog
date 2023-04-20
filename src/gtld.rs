use gemini::gemtext::Builder;

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

/*
 * A collection of RDF statements, order unimportant
 */
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
