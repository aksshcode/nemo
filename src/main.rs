// define url , api key , prompt and header
// send the api request
// parse the output
// print it

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
            {"role":"user","content":"full form of NASA?"}
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
            while let Ok(Some(chunk)) = res.chunk().await {
                let string_chunk = String::from_utf8_lossy(&chunk);
                let string_chunk_lines = string_chunk.lines();
                for line in string_chunk_lines {
                    let prefix_stripped = line.strip_prefix("data: ");
                    match prefix_stripped {
                        Some(stripped) => {
                            let json: Result<serde_json::Value, serde_json::Error> =
                                serde_json::from_str(stripped);
                            match json {
                                Ok(processed_res) => {
                                    let processed_json =
                                        &processed_res["choices"][0]["delta"]["reasoning_content"];
                                    println!("{}", processed_json);
                                }
                                Err(err) => {
                                    println!("Error,{}", err);
                                }
                            }
                        }
                        None => (),
                    }
                }
            }
        }
        Err(err) => {
            println!("Error:{}", err);
        }
    }
}
