use lingua::Language::{English, French, German, Portuguese};
use lingua::LanguageDetectorBuilder;
use std::env;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let message = env::args().skip(1).collect::<Vec<String>>().join(" ");
    let languages = vec![English, Portuguese, German, French];
    let detector = LanguageDetectorBuilder::from_languages(&languages).build();
    let results = detector.detect_multiple_languages_of(message);

    for detected in results.iter() {
        println!("{}", detected.language());
    }

    Ok(())
}
