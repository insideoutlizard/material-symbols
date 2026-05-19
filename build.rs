//! Generates the Rust enum that corresponds to the codepoint file
use std::char::from_u32;
use std::fs::File;
use std::io::Write;

const STRUM_IMPORTS: &str = r#"use strum_macros::EnumString;"#;

const ENUM_DEFINITION_HEADER: &str = r#"
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, EnumString)]
#[strum(serialize_all = "PascalCase")]
pub enum Icon {
"#;

const TO_CHAR_FUNCTION_HEADER: &str = r#"
#[unsafe(no_mangle)]
pub fn char_from_icon(icon: Icon) -> char {
    use self::Icon::*;
    match icon {
"#;

const TO_STR_HEADER: &str = r#"
pub fn icon_to_str(icon: &Icon) -> &'static str {
    use self::Icon::*;
    match *icon {
"#;

fn main() {
    println!("cargo:rerun-if-changed=assets/codepoints.txt");
    println!("cargo:rerun-if-changed=build.rs");

    const CODEPOINTS: &str = include_str!("./assets/codepoints.txt");

    let prefix = if std::env::var("CARGO_FEATURE_ROUNDED").is_ok() {
        "Rounded"
    } else if std::env::var("CARGO_FEATURE_OUTLINED").is_ok() {
        "Outlined"
    } else {
        "Sharp"
    };

    let codepoints = CODEPOINTS
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let mut whitespace_iterator = line.split_whitespace();
            let raw_name = whitespace_iterator.next().unwrap();

            let mut pascal_name = String::from(prefix);
            for component in raw_name.split('_') {
                if component.is_empty() {
                    continue;
                }
                let mut chars = component.chars();
                if let Some(first) = chars.next() {
                    pascal_name.extend(first.to_uppercase());
                    pascal_name.extend(chars);
                }
            }

            let icon_codepoint = whitespace_iterator.next().unwrap();
            let icon_usize = u32::from_str_radix(icon_codepoint, 16).unwrap();
            let icon_char = from_u32(icon_usize).unwrap();

            (pascal_name, icon_char, raw_name.to_string())
        })
        .collect::<Vec<(String, char, String)>>();

    let mut file = File::create("./src/lib.rs").unwrap();

    // 1. Generate core Strum declarations and the Icon enum block
    file.write_all(STRUM_IMPORTS.as_bytes()).unwrap();
    file.write_all(ENUM_DEFINITION_HEADER.as_bytes()).unwrap();
    for (icon_name, _, _) in &codepoints {
        file.write_all(format!("    {},\n", icon_name).as_bytes())
            .unwrap();
    }
    file.write_all(b"}\n\n").unwrap();

    // 2. Generate the character mapping match statement
    file.write_all(TO_CHAR_FUNCTION_HEADER.as_bytes()).unwrap();
    for (icon_name, icon_char, _) in &codepoints {
        file.write_all(format!("        {} => {:?},\n", icon_name, icon_char).as_bytes())
            .unwrap();
    }
    file.write_all(b"    }\n}\n").unwrap();

    // 3. Inject shared trait code directly from static source module
    file.write_all(include_str!("./extra.rs").as_bytes())
        .unwrap();

    // 4. Generate the raw slug identifier text string lookup block
    file.write_all(TO_STR_HEADER.as_bytes()).unwrap();
    for (icon_name, _, raw_name) in &codepoints {
        file.write_all(format!("        {} => {:?},\n", icon_name, raw_name).as_bytes())
            .unwrap();
    }
    file.write_all(b"    }\n}\n").unwrap();

    // 5. Generate bounds indexing definitions and the full registry tracking array
    let total_icons = codepoints.len();
    let array_header = format!(
        r#"
pub const ICON_COUNT: usize = {total_icons};

pub const ALL_ICONS: [Icon; ICON_COUNT] = [
"#
    );

    file.write_all(array_header.as_bytes()).unwrap();
    for (i, (icon_name, _, _)) in codepoints.iter().enumerate() {
        if i > 0 {
            file.write_all(b",\n").unwrap();
        }
        file.write_all(format!("    Icon::{}", icon_name).as_bytes())
            .unwrap();
    }
    file.write_all(b"\n];\n").unwrap();
}
