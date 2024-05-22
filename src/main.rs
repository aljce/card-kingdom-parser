use select::document::Document;
use select::predicate::{Class, Name};
use std::fmt;
use std::io::{self, Read};
use serde::Serialize;
use scryfall::card::Card;

#[derive(Debug)]
struct CardKingdom {
    name: String,
}

#[derive(Debug)]
struct CardMetadata<Card> {
    card: Card
    quality: String,
    quantity: u32,
    price: f32,
    total: f32,
}

#[derive(Debug, Serialize)]
enum Foiling {
    Foil,
    Etched,
}

#[derive(Debug, Serialize)]
struct MoxfieldEntry {
    count: u32,
    name: String,
    edition: String,
    condition: String,
    language: String,
    foil: Option<Foiling>,
    collector_number: String,
    alter: Option<bool>,
    proxy: Option<bool>,
    purchase_price: f32,
}

fn main() {
    let mut html = String::new();
    io::stdin().read_to_string(&mut html).expect("Failed to read from stdin");

    let document = Document::from(html.as_str());

    let mut cards = Vec::new();

    for node in document.find(Name("tr")) {
        let mut cells = node.find(Name("td"));

        if let (Some(desc), Some(style), Some(qty), Some(price), Some(total)) = (
            cells.next(),
            cells.next(),
            cells.next(),
            cells.next(),
            cells.next(),
        ) {
            let card = CardKingdom(desc.text());
            let quality = style.text();
            let quantity: u32 = qty.text().parse().unwrap_or(0);
            let price: f32 = price.text().replace("$", "").parse().unwrap_or(0.0);
            let total: f32 = total.text().replace("$", "").parse().unwrap_or(0.0);

            cards.push(CardMetadata {
                card,
                quality,
                quantity,
                price,
                total,
            });
        }
    }

    println!("{}", Card::named_fuzzy(cards[0].card.name).unwrap());

    for card in &cards {
        println!("{}", card);
    }
}
