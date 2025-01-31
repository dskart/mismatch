use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("🌐 Downloading SCOWL word list...");

    let url = "http://app.aspell.net/create?max_size=60&spelling=US&max_variant=1&diacritic=strip&download=wordlist&encoding=utf-8&format=inline";
    let response = reqwest::get(url).await?;
    println!("✅ Download complete!");

    println!("📝 Processing word list...");
    let text = response.text().await?;

    // Process and clean the words
    let words: Vec<String> = text
        .lines()
        .filter_map(|line| {
            let word = line.trim();
            if !word.is_empty()
                && word.chars().all(|c| c.is_alphabetic())
                && !word.starts_with(|c: char| c.is_uppercase())
                && 3 < word.len()
                && word.len() < 10
            {
                Some(word.to_string())
            } else {
                None
            }
        })
        .collect();

    println!("📊 Found {} valid words", words.len());

    println!("💾 Saving word list...");
    let dir_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("gen");
    std::fs::create_dir_all(&dir_path)?;

    let file_path = dir_path.join("english_words.txt");
    let mut file = File::create(file_path)?;
    for word in words {
        writeln!(file, "{}", word)?;
    }

    println!("✨ Word list successfully saved to english_words.txt!");
    Ok(())
}
