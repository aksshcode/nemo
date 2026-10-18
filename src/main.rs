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
            {"role":"user","content":"Full form of NASA?"}
        ]
    }"#;

    let response = client
        .post(url)
        .header("Authorization", format!("Bearer {api_key}"))
        .header("Content-type", "Application/json")
        .body(payload)
        .send()
        .await;

    match response {
        Ok(res) => {
            let res_text = res.text().await;
            match res_text {
                Ok(text) => {
                    let json: Result<serde_json::Value, serde_json::error::Error> =
                        serde_json::from_str(&text);
                    match json {
                        Ok(json_value) => {
                            let content = &json_value["choices"][0]["message"]["content"];
                            let content_str_version = content.as_str();
                            match content_str_version {
                                Some(pretty) => {
                                    termimad::print_text(pretty);
                                }
                                None => {
                                    println!(
                                        "Problem prettying the ai reply, fallback to normal instead."
                                    );
                                    println!("{}", content)
                                }
                            }
                        }
                        Err(err) => {
                            println!("Error:{}", err)
                        }
                    }
                }
                Err(err) => {
                    println!("Error:{}", err);
                }
            }
        }
        Err(err) => {
            println!("Error:{}", err);
        }
    }
}
