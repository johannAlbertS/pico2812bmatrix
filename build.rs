use std::env::current_dir;

const TEXT: &str = "Rutendo Gandi";
//const TEXT: &str = "custom apps fur die matrix sind willkommen githum johannAlbertS pico2812bmatrix"
// const TEXT: &str = "The one with the braid";
// const TEXT: &str = "Simon Duckert";
// const TEXT: &str = "Anke Domsheidt Berg";
// const TEXT: &str = "Enny";
// const TEXT: &str = "Bonnie Mehring";
// const TEXT: &str = "Benedikt";
// const TEXT: &str = "Katja Lauth";

const OUTPUT_FILE_NAME: &str = "config.rs";

fn main() {
    let mut output = String::new();
    let text = TEXT.to_uppercase();
    let words: Vec<&str> = text.split(' ').collect();
    let mut buffer_len = 3;
    output.push_str(&format!("pub const TEXT: [char; {}] = [", text.len()));
    for (i, word) in words.iter().enumerate() {
        for letter in word.chars() {
            output.push_str(&format!("\'{letter}\', "));
            buffer_len += get_letter_buffer_len(letter);
        }
        if words.len() - 1 == i {
            break;
        }
        output.push_str("\' \', ");
        buffer_len += 3;
    }
    output.push_str("];\n\n");
    output.push_str("#[macro_export]\nmacro_rules! textbuffer {\n    () => {\n        [\n");
    for _i in 0..buffer_len + 11 {
        output.push_str("            textbuffer_line!(),\n")
    }
    output.push_str("        ]\n    }\n}\n");
    let output_file = current_dir().unwrap().join("src").join(OUTPUT_FILE_NAME);
    println!("{}", output);
    std::fs::write(output_file, output).unwrap();
}

fn get_letter_buffer_len(letter: char) -> i32 {
    match letter {
        'k' | 'm' | 'n' | 'q' | 't' | 'v' | 'w' | 'x' | 'y' | 'z' => 6,
        'i' => 4,
        '.' => 2,
        _ => 5,
    }
}
