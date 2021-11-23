use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    // This is ok, except when it comes to facing glyph characters...
    // input.chars().rev().collect::<String>()
    
    // Use unicode_segmentation crate which provides implementation for iterating over grapheme clusters.
    input.graphemes(true).rev().collect::<String>()
}