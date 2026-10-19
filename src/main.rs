// define url , api key , prompt and header
// send the api request
// parse the output
// print it

use std::io::Write;

// sending http request requires reqwest which requires an async runtime
#[tokio::main]
async fn main() {
    //define a client to use i guess , but i have no idea why i use clien there i  could just do
    //reqwest::client::post() or smth
    let client = reqwest::Client::new();

    let url = "https://integrate.api.nvidia.com/v1/chat/completions";
    let api_key = "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";
    let payload = r#"{
        "model":"nvidia/nemotron-3-nano-omni-30b-a3b-reasoning",
        "messages":[
            {"role":"user","content":"full form of NASA and ISRO?"}
        ],
        "stream":true
    }"#;

    let response = client
        .post(url)
        .header("Authorization", format!("Bearer {api_key}"))
        .header("Content-type", "Application/json")
        .body(payload)
        .send()
        .await;

    match response {
        Ok(mut res) => {
            if !res.status().is_success() {
                eprintln!("HTTP Error {}: {}", res.status(), res.text().await.unwrap_or_default());
                return;
            }

            'stream: while let Ok(Some(chunk)) = res.chunk().await {
                let string_chunk = String::from_utf8_lossy(&chunk);
                let string_chunk_lines = string_chunk.lines();
                for line in string_chunk_lines {
                    let chop_the_data_prefix = line.strip_prefix("data: ");
                    match chop_the_data_prefix {
                        Some(chopped) => {
                            if chopped.trim() == "[DONE]" {
                                break 'stream;
                            }
                            let convert_to_json_attempt: Result<
                                serde_json::Value,
                                serde_json::Error,
                            > = serde_json::from_str(chopped);
                            match convert_to_json_attempt {
                                Ok(json) => {
                                    let content = &json["choices"][0]["delta"];
                                    let delta = content;
                                    if let Some(thought) = delta["reasoning_content"].as_str() {
                                        print!("\x1b[90m{thought}\x1b[0m");
                                        let _ = std::io::stdout().flush();
                                    }
                                    if let Some(answer) = delta["content"].as_str() {
                                        print!("{answer}");
                                        let _ = std::io::stdout().flush();
                                    }
                                }
                                Err(err) => eprintln!("JSON Error: {}", err),
                            }
                        }
                        None => (),
                    }
                }
            }
            println!();
        }
        Err(err) => {
            eprintln!("Network Error: {}", err);
        }
    }
}
