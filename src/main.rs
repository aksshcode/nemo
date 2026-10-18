// use reqwest::Response;

#[tokio::main]
async fn main() {
    let client = reqwest::Client::new();
    let api_key = "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";
    let payload = r#"{
        "model": "nvidia/nemotron-3-nano-omni-30b-a3b-reasoning",
        "messages": [
            {"role": "user", "content": "similarities in hindi and japanese"}
        ]
    }"#;
    let res = client
        .post("https://integrate.api.nvidia.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {api_key}"))
        .header("Content-Type", "application/json")
        .body(payload)
        .send()
        .await;

    match res {
        Ok(response) => {
            let body = response.text().await.unwrap();
            let parsed: serde_json::Value = serde_json::from_str(&body).unwrap();
            let ai_response_text = parsed["choices"][0]["message"]["content"].as_str().unwrap();
            // println!("{}", ai_response_text);
            termimad::print_text(ai_response_text);
        }
        Err(err) => {
            println!("error:{}", err);
        }
    }
}

// api key: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
