use std::cmp::Ordering;

use gemini::gemtext::Builder;
use gemini::request::{Gemini, Request};

fn triple(s: String, p: String, o: String) {
    (s, p, o);
}

fn client(uri: &str) {
    let req = Request::<Gemini>::from_uri(uri).expect("Could not get a Gemini request!");
    println!("Is {} a Gemini request? {}", uri, req.is_gemini_request());
}

fn main() {
    println!("Welcome to my Rust project bringing Linked Data over to the Small Web!");

    client("gemini://gemini.circumlunar.space");

    let rdf = "http://www.w3.org/1999/02/22-rdf-syntax-ns#";

    let rdftype = format!("{}type", rdf);

    let s = "https://orcid.org/0000-0002-9272-908X";
    let p = "http://www.w3.org/1999/02/22-rdf-syntax-ns#type";
    let o = "http://xmlns.com/foaf/0.1/Person";

    match p.cmp(&rdftype) {
        Ordering::Less => println!("[FAILURE] Not an rdf:type"),
        Ordering::Greater => println!("[FAILURE] Not an rdf:type"),
        Ordering::Equal => println!("[SUCCESS] This is an rdf:type expression!"),
    }
    triple(s.to_string(), p.to_string(), o.to_string());
    println!("{} {} {} .", s, p, o);

    let ld = Builder::new()
        .link(s, Some(s))
        .link(p, Some(p))
        .link(o, Some(o))
        .line();

    println!("{}", &ld);
}
