//! Generates the Rust enum that corresponds to the codepoint file
use std::char::from_u32;
use std::fs::File;
use std::io::Write;

const RUST_CODE_START_1: &str = "
/// Icon containing all possible icon names as enum discriminants
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Icon {\n";
const RUST_CODE_END_1: &str = "}\n\n";
const RUST_CODE_START_2: &str = "
/// Converts an `Icon` to a `char`. Same as `format!(\"{}\", icon)` or `icon.to_char()`.
#[unsafe(no_mangle)]
pub fn icon_to_char(icon: Icon) -> char {
    use self::Icon::*;
    match icon {
";
const RUST_CODE_END_2: &str = "    }\n}\n";
const RUST_CODE_3: &str = include_str!("./extra.rs");
const RUST_CODE_START_4: &str = "
/// Get icon HTML name
pub fn icon_to_html_name(icon: &Icon) -> &'static str {
    use self::Icon::*;
    match *icon {
";
const RUST_CODE_END_4: &str = r#"    }
}
"#;

const RUST_CODE_START_5: &str = "
/// Total number of Material Icons
pub const ICON_COUNT: usize = ";

const RUST_CODE_MID_5: &str = ";

/// Array of all Material Icons for iteration
///
/// Use this to register all icons at startup:
/// ```rust
/// use material_icons::{ALL_ICONS, icon_to_char, icon_to_html_name};
/// for icon in ALL_ICONS.iter() {
///     let char_code = icon_to_char(*icon);
///     let name = icon_to_html_name(icon);
///     // ... register icon
/// }
/// ```
pub const ALL_ICONS: [Icon; ICON_COUNT] = [\n";

const RUST_CODE_END_5: &str = "];\n";

fn main() {
    println!("cargo:rerun-if-changed=assets/codepoints.txt");
    println!("cargo:rerun-if-changed=build.rs");

    const CODEPOINTS: &str = include_str!("./assets/codepoints.txt");

    // get prefix from feature flag
    let prefix = if std::env::var("CARGO_FEATURED_ROUNDED").is_ok() {
        "Rounded"
    } else if std::env::var("CARGO_FEATURED_OUTLINED").is_ok() {
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

            // Convert snake_case to PascalCase, with prefix
            let mut pascal_name = String::from(prefix);
            for component in raw_name.split("_") {
                if component.is_empty() {
                    continue;
                }
                let mut chars = component.chars();
                if let Some(first) = chars.next() {
                    pascal_name.extend(first.to_uppercase());
                    pascal_name.extend(chars);
                }
            }

            // Parse and convert codepoint
            let icon_codepoint = whitespace_iterator.next().unwrap(); // e84d => \u{e84d}
            let icon_usize = u32::from_str_radix(icon_codepoint, 16).unwrap();
            let icon_char = from_u32(icon_usize).unwrap();

            (pascal_name, icon_char, raw_name.to_string())
        })
        .collect::<Vec<(String, char, String)>>();

    let mut file = File::create("./src/lib.rs").unwrap();

    // -- part 1: create the enum
    file.write(RUST_CODE_START_1.as_bytes()).unwrap();
    for (icon_name, _, _) in &codepoints {
        let enum_str = format!("    {},\n", icon_name);
        file.write(enum_str.as_bytes()).unwrap();
    }
    file.write(RUST_CODE_END_1.as_bytes()).unwrap();

    // -- part 2: match on the enum
    file.write(RUST_CODE_START_2.as_bytes()).unwrap();
    for (icon_name, icon_char, _) in &codepoints {
        let enum_str = format!("        {} => {:?},\n", icon_name, icon_char);
        file.write(enum_str.as_bytes()).unwrap();
    }
    file.write(RUST_CODE_END_2.as_bytes()).unwrap();

    // -- part 3: convenience code
    file.write(RUST_CODE_3.as_bytes()).unwrap();

    // -- part 4: match on the enum
    file.write(RUST_CODE_START_4.as_bytes()).unwrap();
    for (icon_name, _, name) in &codepoints {
        let enum_str = format!("        {} => {:?},\n", icon_name, name);
        file.write(enum_str.as_bytes()).unwrap();
    }
    file.write(RUST_CODE_END_4.as_bytes()).unwrap();

    // -- part 5: generate ALL_ICONS array
    file.write(RUST_CODE_START_5.as_bytes()).unwrap();
    file.write(format!("{}", codepoints.len()).as_bytes())
        .unwrap();
    file.write(RUST_CODE_MID_5.as_bytes()).unwrap();
    for (i, (icon_name, _, _)) in codepoints.iter().enumerate() {
        if i > 0 {
            file.write(b",\n").unwrap();
        }
        let enum_str = format!("    Icon::{}", icon_name);
        file.write(enum_str.as_bytes()).unwrap();
    }
    file.write(b"\n").unwrap();
    file.write(RUST_CODE_END_5.as_bytes()).unwrap();
}
