use std::cmp::Ordering;

use gemini::gemtext::Builder;
use gemini::request::{Gemini, Request};
use log::info;

const NS_FOAF: &str = "http://xmlns.com/foaf/0.1/";
const NS_RDF: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#";

fn stmt(s: String, p: String, o: String) -> (String, String, String) {
    (s, p, o)
}

// Doesn't seem to do anything, why? A reference / call-by-value problem?
fn to_gemtext( triples: Vec<(String, String, String)> ) -> gemini::Builder {
    let gemtext = Builder::new();
    for (s, p, o) in triples.iter() {
        gemtext.clone()
            .link(s.clone(), Some(s))
            .link(p.clone(), Some(p))
            .link(o.clone(), Some(o))
            .line();
    }
    println!("{}",gemtext);
    gemtext
}

fn client(uri: &str) {
    let req = Request::<Gemini>::from_uri(uri).expect("Could not get a Gemini request!");
    println!("Is {} a Gemini request? {}", uri, req.is_gemini_request());
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

    match p.cmp(&rdftype) {
        Ordering::Less => println!("[FAILURE] Not an rdf:type"),
        Ordering::Greater => println!("[FAILURE] Not an rdf:type"),
        Ordering::Equal => println!("[SUCCESS] This is an rdf:type expression!"),
    }
    println!("{} {} {} .", s, p, o);
    stmt(String::from(s),String::from(p),o.clone());

    let mut vec = Vec::<(String, String, String)>::new();
    vec.push(stmt(String::from(s),String::from(p),o.clone()));
    println!("{}",to_gemtext(vec));
    
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
