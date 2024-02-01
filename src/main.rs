use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let port_number = "51581";
    let session_id = "a0201acbf42d389d612062214520e1f2";
    let url = format!("http://localhost:{}/session/{}/screenshot", port_number, session_id);
    let resp = reqwest::get(url)  // INFO: 240201 パラメタ設定すると、base64 でとれた。
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    println!("{:#?}", resp);  // EDIT: 240201 base64 を保存できるようにする。
    Ok(())
}