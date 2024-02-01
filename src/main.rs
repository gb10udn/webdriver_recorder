use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("http://localhost:51581/session/a0201acbf42d389d612062214520e1f2/screenshot")  // INFO: 240201 パラメタ設定すると、base64 でとれた。
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    println!("{:#?}", resp);
    Ok(())
}