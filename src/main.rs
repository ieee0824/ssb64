use arboard::Clipboard;
use base64::{engine::general_purpose::STANDARD, Engine as _};
use std::fs;
use std::process::Command;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("マウスでドラッグしてスクリーンショットの範囲を選択してください...");

    // 一時ファイルパスを生成（screencaptureはpng拡張子が必要）
    let temp_path = std::env::temp_dir().join(format!("ssb64_{}.png", std::process::id()));
    
    // macOSのscreencaptureコマンドでインタラクティブ選択
    // -i: インタラクティブモード（マウスで範囲選択）
    // -s: 選択モード
    let status = Command::new("screencapture")
        .arg("-i")
        .arg(&temp_path)
        .status()?;

    if !status.success() {
        eprintln!("スクリーンショットの取得に失敗しました");
        return Ok(());
    }

    // ファイルが存在するか確認（ユーザーがキャンセルした場合は存在しない）
    if !temp_path.exists() || fs::metadata(&temp_path)?.len() == 0 {
        println!("スクリーンショットがキャンセルされました");
        return Ok(());
    }

    // 画像をバイナリで読み込む
    let image_data = fs::read(&temp_path)?;
    
    // base64エンコード
    let base64_data = STANDARD.encode(&image_data);
    
    // クリップボードにコピー
    let mut clipboard = Clipboard::new()?;
    clipboard.set_text(&base64_data)?;
    
    println!("✓ スクリーンショットをbase64エンコードしてクリップボードにコピーしました");
    println!("  データサイズ: {} bytes", image_data.len());
    println!("  base64サイズ: {} chars", base64_data.len());
    
    Ok(())
}
