/*!
 * __This code will evolve into a software library.__
 *
 * The main method is primarily for demo/showcase, and for testing before I find out
 * how tests are done in Rust :)
 */

mod gtld;
mod namespace;
mod util;

use gtld::{Document, Statement};
use log::info;
use namespace::{NS_FOAF, NS_ORCID, NS_RDF};
use std::cmp::Ordering;
use util::to_gemini_uri;

fn stmt(s: String, p: String, o: String) -> Statement {
    let triple = Statement::new(s, p, o, None, None);
    triple
}

fn _print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}

fn main() {
    println!("Welcome to my Rust project bringing Linked Data over to the Small Web!");

    // Won't work until I configure a logger.
    info!("1. Create a sample statement.");

    let rdftype = format!("{}type", NS_RDF);

    let s = format!("{NS_ORCID}0000-0002-9272-908X");
    let p = "http://www.w3.org/1999/02/22-rdf-syntax-ns#type";
    let o = NS_FOAF.to_owned() + "Person";
    println!("{} {} {} .", s, p, o);

    info!("2. Check that the sample statement has the expected predicate.");
    match p.cmp(&rdftype) {
        Ordering::Less => println!("[FAILURE] Not an rdf:type"),
        Ordering::Greater => println!("[FAILURE] Not an rdf:type"),
        Ordering::Equal => println!("[SUCCESS] This is an rdf:type expression!"),
    }

    info!("3. Create a new RDF document from that and another statement.");
    let mut rdf = Document::new();
    rdf.add(stmt(s.clone(), String::from(p), o.clone()));
    rdf.add(stmt(
        String::from(s),
        format!("{NS_FOAF}name"),
        String::from("Alessandro Adamou"),
    ));

    info!("4. Convert to Gemtext and print.");
    println!("Printing an RDF document of {} statements...", rdf.len());
    println!();
    println!("{}", rdf.to_gemtext());

    println!("Demo over. You can run `cargo test` to test other features.");
}
