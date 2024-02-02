use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use base64::prelude::*;
use clap::Parser;
use std::time::Instant;


/// webdriver の録画を実行する関数。
/// コマンドライン引数の呼び出し方が、cargo で実行と、.exe パスを渡す で -- の使い方が微妙に異なる点に注意する。
/// Ex.1 cargo run -- -p 65478 -s f33e6812e7efb6e926b8801cf60f94e8 (cargo 実行時は、-- 必要)
/// Ex.2 .\target\debug\webdriver-recoreder.exe -p 65478 -s f33e6812e7efb6e926b8801cf60f94e8 (.exe 実行時は、-- 不要)
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start = Instant::now();
    let is_debug_mode = true;  // FIXME: 240203 リリース時には false に戻すこと。
    let args = Args::parse();
    let url = format!("http://localhost:{}/session/{}/screenshot", args.port_number, args.session_id);

    if is_debug_mode {print_duration(start)};

    let resp = reqwest::get(url)  // INFO: 240201 パラメタ設定すると、base64 でとれた。
        .await?
        .json::<HashMap<String, String>>()
        .await?;

    if is_debug_mode {print_duration(start)};  // INFO: 240203 ここで全体の 95% 近くの時間がかかる。(= webdriver api の応答待ちが律速。)
    
    if let Some(base64_string) = resp.get("value") {  // TODO: 240202 while true でループさせて、エラーが連続で３回とか出たら処理落とすとかにするといいかも？
        let binary_data = BASE64_STANDARD.decode(base64_string).unwrap();
        let file_path = "output.png";  // TODO: 240203 上書きするのではなくて、名前を微妙に変更するようにする。(開始からの時間をファイル名に入れてもいいかも？)
        let mut file = File::create(file_path).expect("Unable to create file");
        file.write_all(&binary_data).expect("Unable to write data to file");
        print_duration(start);
    }

    if is_debug_mode {print_duration(start)};
    
    Ok(())
}

fn print_duration(start: Instant) {
    let end = start.elapsed();
    println!("{}.{:03} second passed", end.as_secs(), end.subsec_nanos() / 1_000_000);
}


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// webdriver のポート番号
    #[arg(short = 'p', long)]
    port_number: String,

    /// webdriver のセッション ID
    #[arg(short = 's', long)]
    session_id: String,
}