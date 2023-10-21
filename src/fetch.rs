use reqwest;

pub fn fetch_random_word() -> Result<Vec<String>, reqwest::Error> {
    let api_url = "https://random-word-api.herokuapp.com/word?number=1";
    let response = reqwest::blocking::get(api_url)
        .unwrap();
    let words: Vec<String> = response.json()?;

    Ok(words)
}