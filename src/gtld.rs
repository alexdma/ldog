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

impl Document {
    pub fn new() -> Document {
        Document { statements: Vec::<Statement>::new() }
    }
    
    pub fn add(mut self, s: Statement) {
        self.statements.push(s);
    }
}