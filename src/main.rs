use std::collections::HashMap;
use std::fs::File;
use std::fs;
use std::io::Write;
use base64::prelude::*;
use clap::Parser;


/// webdriver の録画を実行する関数。
/// コマンドライン引数の呼び出し方が、cargo で実行と、.exe パスを渡す で -- の使い方が微妙に異なる点に注意する。
/// Ex.1 cargo run -- -p 65478 -s f33e6812e7efb6e926b8801cf60f94e8 (cargo 実行時は、-- 必要)
/// Ex.2 .\target\debug\webdriver-recorder.exe -p 65478 -s f33e6812e7efb6e926b8801cf60f94e8 (.exe 実行時は、-- 不要)
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // [START] set params
    let base_dst_dir = "./dst";
    fs::create_dir_all(base_dst_dir)?;
    // [END] set params

    let args = Args::parse();
    let url = format!("http://localhost:{}/session/{}/screenshot", args.port_number, args.session_id);
    let mut image_index = 0;
    let mut failure_num = 0;
    loop {
        let temp = reqwest::get(&url).await;

        match temp {
            Ok(resp) => {
                let resp = resp
                    .json::<HashMap<String, String>>()
                    .await?;
        
                if let Some(base64_string) = resp.get("value") {
                    let binary_data = BASE64_STANDARD.decode(base64_string).unwrap();
                    let file_path = format!("{}/{}.png", base_dst_dir, image_index);
                    let mut file = File::create(file_path).expect("Unable to create file");
                    file.write_all(&binary_data).expect("Unable to write data to file");
                }
                image_index += 1;
                failure_num = 0;
            },
            Err(_) => {
                failure_num += 1;
            },
        }

        if failure_num > 2 {
            break;
        }
    }
    Ok(())
}


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// webdriver のポート番号
    #[arg(short = 'p', long)]  // INFO: 240204 short を設定すると、-p 54321 などのように呼び出せる。
    port_number: String,

    /// webdriver のセッション ID
    #[arg(short = 's', long)]
    session_id: String,
}