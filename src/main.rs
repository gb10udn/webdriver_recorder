use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use base64::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let port_number = "51581";  // TODO: 240201 コマンド引数で受け取れるようにする。
    let session_id = "a0201acbf42d389d612062214520e1f2";  // TODO: 240201 コマンド引数で受け取れるようにする。
    let url = format!("http://localhost:{}/session/{}/screenshot", port_number, session_id);
    
    let resp = reqwest::get(url)  // INFO: 240201 パラメタ設定すると、base64 でとれた。
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    
    println!("{:#?}", resp.get("value"));  // EDIT: 240201 base64 を保存できるようにする。

    if let Some(base64_string) = resp.get("value") {
        let binary_data = BASE64_STANDARD.decode(base64_string).unwrap();
        let file_path = "output.png";
        let mut file = File::create(file_path).expect("Unable to create file");
        file.write_all(&binary_data).expect("Unable to write data to file");
    }
    
    Ok(())
}