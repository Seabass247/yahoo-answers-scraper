use scraper::{Html, Selector};

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    // Make a GET request to Yahoo Answers' latest questions asked page to retrieve its html body text.
    let body = reqwest::get("https://answers.yahoo.com/answer")
        .await?
        .text()
        .await?;

    // Parses string of HTML as a document
    let fragment = Html::parse_document(&body);
    // Parses based on the CSS selector ".Clr-b", which is found by using the "SelectorGadget"
    // extension in Chrome on a question in the answers page.
    let questions = Selector::parse(".QuestionCard__title___1DKC-").unwrap();

    // Iterate over elements matching the selector
    for question in fragment.select(&questions) {
        // Grab the question text from the element's first (index 0) descendent text node
        let question_txt = question.text().collect::<Vec<_>>()[0];
        // Only text containing "?" is a valid question, otherwise it's a category or label text
        if question_txt.contains("?") {
            // Do something with the question.
            println!("{}", question_txt);
        }
    }

    Ok(())
}
