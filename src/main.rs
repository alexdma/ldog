mod gtld;
mod namespace;

use gemini::request::{Gemini, Request};
use gtld::{Document, Statement};
use log::info;
use namespace::{NS_FOAF, NS_RDF};
use std::cmp::Ordering;

fn stmt(s: String, p: String, o: String) -> Statement {
    let triple = Statement::new(s, p, o);
    triple
}

fn client(uri: &str) {
    let req = Request::<Gemini>::from_uri(uri).expect("Could not get a Gemini request!");
    println!("Is {} a Gemini request? {}", uri, req.is_gemini_request());
    assert!(req.is_gemini_request());
}

fn _print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
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
    let mut rdf = Document::new();
    rdf.add(stmt(String::from(s), String::from(p), o.clone()));
    rdf.add(stmt(
        String::from(s),
        format!("{NS_FOAF}name"),
        String::from("Alessandro Adamou"),
    ));

    println!("Printing an RDF document of {} statements...", rdf.len());
    println!();
    println!("{}", rdf.to_gemtext());
}
