use std::collections::HashMap;
use std::io::Write;
use std::fs::File;
use std::fs;
use std::process::Command;
use std::process::Stdio;
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
                // [START] save image
                let resp = resp
                    .json::<HashMap<String, String>>()
                    .await;

                match resp {
                    Ok(resp) => {
                        if let Some(base64_string) = resp.get("value") {
                            let binary_data = BASE64_STANDARD.decode(base64_string).unwrap();
                            let file_path = format!("{}/image{:05}.png", base_dst_dir, image_index);
                            let mut file = File::create(file_path).expect("Unable to create file");
                            file.write_all(&binary_data).expect("Unable to write data to file");
                        }
                        image_index += 1;
                        failure_num = 0;
                    },
                    Err(_) => {
                        failure_num += 1;   // INFO: 240204 途中で webdriver が切断された場合？
                    }
                }
                // [END] save image
            },
            Err(_) => {
                failure_num += 1;  // INFO: 240204 最初からパラメタ失敗した場合？
            },
        }

        if failure_num > 2 {
            break;
        }
    }
    create_movie(&base_dst_dir);
    // TODO: 240204 ./dst フォルダを削除する。
    Ok(())
}


fn create_movie(base_dir: &str) {  // TODO: 240204 出力先のパスをもう少し使いやすくする。
    let output = Command::new("ffmpeg")
        .args(["-i", &format!("{}/image%05d.png", base_dir), "-c:v", "libx264", "output.mp4"])  // TODO: 240204 fps を最適化する。
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .unwrap();
    println!("{:?}", output);
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