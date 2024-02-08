use std::collections::HashMap;
use std::io::Write;
use std::fs::File;
use std::fs;
use std::process::Command;
use std::process::Stdio;
use std::time;
use base64::prelude::*;
use clap::Parser;


/// webdriver の録画を実行する関数。
/// コマンドライン引数の呼び出し方が、cargo で実行と、.exe パスを渡す で -- の使い方が微妙に異なる点に注意する。
/// Ex.1 cargo run -- -p {{ port_number}} -s {{ session_id }} (cargo 実行時は、-- 必要)
/// Ex.2 .\target\debug\webdriver-recorder.exe -p {{ port_number}} -s {{ session_id }} (.exe 実行時は、-- 不要)
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // [START] set params
    let base_dst_dir = "./webdriver_recorder_dist";
    fs::create_dir_all(base_dst_dir)?;
    // [END] set params

    let args = Args::parse();
    let url = format!("http://localhost:{}/session/{}/screenshot", args.port_number, args.session_id);
    let mut image_index = 0;
    let mut failure_num = 0;

    let start_time = time::Instant::now();
    loop {
        let temp = reqwest::get(&url).await;

        match temp {
            Ok(resp) => {
                let resp = resp
                    .json::<HashMap<String, String>>()
                    .await;

                match resp {
                    Ok(resp) => {
                        if let Some(base64_string) = resp.get("value") {
                            // [START] save image
                            let binary_data = BASE64_STANDARD.decode(base64_string).unwrap();
                            let file_path = format!("{}/image{:05}.png", base_dst_dir, image_index);  // FIXME: 240204 0 埋めの桁数が連動している箇所があるので、注意する。
                            let mut file = File::create(file_path).expect("Unable to create file");
                            file.write_all(&binary_data).expect("Unable to write data to file");
                            // [END] save image
                        }
                        image_index += 1;
                        failure_num = 0;
                    },
                    Err(_) => {
                        failure_num += 1;   // INFO: 240204 途中で webdriver が切断された場合？
                    }
                }
            },
            Err(_) => {
                failure_num += 1;  // INFO: 240204 最初からパラメタ失敗した場合？
            },
        }

        if failure_num > 2 {
            break;
        }
    }
    let duration = start_time.elapsed().as_secs();
    let fps = (image_index as f64 / duration as f64).round() as usize;

    create_movie(&base_dst_dir, &fps);
    // TODO: 240204 ./dst フォルダを削除する。 (.png ファイルを削除する。)
    Ok(())
}


fn create_movie(base_dir: &str, fps: &usize) {
    let output = Command::new("ffmpeg")  // FIXME: 240204 ffmpeg がパスが通っていない場合にエラーをあげれるようにする？
        .args(["-i", &format!("{}/image%05d.png", base_dir), "-c:v", "libx264", "-r", &fps.to_string(), "output.mp4"])
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
    #[arg(short = 'p', long)]  // INFO: 240204 short を設定すると、-p {{ port_number }} などのように呼び出せる。
    port_number: String,

    /// webdriver のセッション ID
    #[arg(short = 's', long)]
    session_id: String,
}