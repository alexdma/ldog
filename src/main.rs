mod gtld;

use gemini::gemtext::Builder;
use gemini::request::{Gemini, Request};
use gtld::{Document, Statement};
use log::info;
use std::cmp::Ordering;

const NS_FOAF: &str = "http://xmlns.com/foaf/0.1/";
const NS_RDF: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#";

fn stmt(s: String, p: String, o: String) -> Statement {
    let triple = Statement::new(s, p, o);
    triple
}

// Doesn't seem to do anything, why? A reference / call-by-value problem?
fn to_gemtext(triples: Document) -> gemini::Builder {
    let gemtext = Builder::new();
    for stmt in triples.statements.iter() {
        let s = &stmt.subject;
        let p = &stmt.predicate;
        let o = &stmt.object;
        gemtext
            .clone()
            .link(&*s, Some(s))
            .link(&*p, Some(p))
            .link(&*o, Some(o))
            .line();
    }
    println!("{}", gemtext);
    gemtext
}

fn client(uri: &str) {
    let req = Request::<Gemini>::from_uri(uri).expect("Could not get a Gemini request!");
    println!("Is {} a Gemini request? {}", uri, req.is_gemini_request());
    assert!(req.is_gemini_request());
}

fn _print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn main() {
    println!("Welcome to my Rust project bringing Linked Data over to the Small Web!");

    const GEMURL: &str = "gemini://gemini.circumlunar.space";

    // Won't work until I configure a logger.
    info!("First let's build a Request to {}", GEMURL);

    client(GEMURL);

    let rdftype = format!("{}type", NS_RDF);

    let s = "https://orcid.org/0000-0002-9272-908X";
    let p = "http://www.w3.org/1999/02/22-rdf-syntax-ns#type";
    let o = NS_FOAF.to_owned() + "Person";
    _print_type_of(&s);
    _print_type_of(&o);

    println!("{} {} {} .", s, p, o);
    match p.cmp(&rdftype) {
        Ordering::Less => println!("[FAILURE] Not an rdf:type"),
        Ordering::Greater => println!("[FAILURE] Not an rdf:type"),
        Ordering::Equal => println!("[SUCCESS] This is an rdf:type expression!"),
    }
    stmt(String::from(s), String::from(p), o.clone());

    //let mut vec = Vec::<Statement>::new();
    //vec.push(stmt(String::from(s), String::from(p), o.clone()));
    let mut rdf = Document::new();
    rdf = rdf.add(stmt(String::from(s), String::from(p), o.clone()));

    // PRINTS NOTHING
    println!("{}", to_gemtext(rdf));

    let ld = Builder::new()
        .link(s, Some(s))
        .link(p, Some(p))
        .link(o.clone(), Some(o))
        .line();
    println!("{}", &ld);

    {
        let p = format!("{NS_FOAF}name");
        let o = "Alessandro Adamou";
        println!("{} {} {} .", s, p, o);
        let ld = Builder::new()
            .link(s, Some(s))
            .link(p.clone(), Some(p))
            .link(o.clone(), Some(o))
            .line();
        println!("{}", &ld);
    }
}
