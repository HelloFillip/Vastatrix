extern crate xml;

use std::fs::File;

use self::xml::reader::EventReader;
use self::xml::reader::events::*;

fn indent(size: usize) -> String {
    const INDENT: &'static str = "    ";
    (0..size).map(|_| INDENT)
             .fold(String::with_capacity(size*INDENT.len()), |r, s| r + s)
}

pub fn load() {
    let file = File::open("vastatrix.config").unwrap();

    let mut parser = EventReader::new(file);
    let mut depth = 0;

    for event in parser.events() {
        match event {
            XmlEvent::StartElement { name, .. } => {
                println!("{}+{}", indent(depth), name);
                depth += 1;
            }
            XmlEvent::EndElement { name } => {
                depth -= 1;
                println!("{}-{}", indent(depth), name);
            }
            XmlEvent::Error(e) => {
                println!("Error: {}", e);
                break;
            }
            _ => {}
        }
    }
}
