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

            let mut full_response = String::new();

            'stream: loop {
                match res.chunk().await {
                    Ok(Some(chunk)) => {
                        let string_chunk = String::from_utf8_lossy(&chunk);
                        for line in string_chunk.lines() {
                            if let Some(chopped) = line.strip_prefix("data: ") {
                                if chopped.trim() == "[DONE]" {
                                    break 'stream;
                                }
                                match serde_json::from_str::<serde_json::Value>(chopped) {
                                    Ok(json) => {
                                        let delta = &json["choices"][0]["delta"];
                                        if let Some(thought) = delta["reasoning_content"].as_str() {
                                            print!("\x1b[90m{thought}\x1b[0m");
                                            let _ = std::io::stdout().flush();
                                        }
                                        if let Some(answer) = delta["content"].as_str() {
                                            print!("{answer}");
                                            let _ = std::io::stdout().flush();
                                            full_response.push_str(answer);
                                        }
                                    }
                                    Err(err) => eprintln!("JSON Error: {}", err),
                                }
                            }
                        }
                    }
                    Ok(None) => break,
                    Err(err) => {
                        eprintln!("Socket Error: {}", err);
                        break;
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
