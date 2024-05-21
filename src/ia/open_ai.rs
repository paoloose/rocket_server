
pub mod chat_ai {
    use openai_api_rs::v1::api::Client;
    use openai_api_rs::v1::chat_completion::{self, ChatCompletionRequest};
    use openai_api_rs::v1::common::GPT4;
    use dotenv::dotenv;

    pub fn chat(message: String, api_token: String) -> Result<Option<String>, Box<dyn std::error::Error>> {
        dotenv().ok();
        let client = Client::new(api_token);

        println!("{}", &message);

        let req = ChatCompletionRequest::new(
            GPT4.to_string(),
            vec![chat_completion::ChatCompletionMessage {
                role: chat_completion::MessageRole::user,
                content: chat_completion::Content::Text(message),
                name: None,
            }],
        );

        let result = client.chat_completion(req)?;
        let string = &result.choices[0].message.content;
        // println!("Content: {:?}", result.choices[0].message.content);
        // println!("Response Headers: {:?}", result.headers);

        Ok(string.clone())
    }
}
