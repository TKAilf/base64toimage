use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use std::fs::File;
use std::io::Write;

fn main() {
    // Base64エンコードされた画像データ
    let base64_data = "xxx";

    // Base64デコード
    let decoded_data = match STANDARD.decode(base64_data) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Failed to decode Base64 data '{}': {}", base64_data, e);
            return;
        }
    };

    // デコードしたデータを画像ファイルとして保存
    let output_path = "output_image.png";
    match save_to_file(&decoded_data, output_path) {
        Ok(_) => println!("Image saved to {}", output_path),
        Err(e) => eprintln!("Failed to save image: {}", e),
    }
}

// バイトデータをファイルに保存する関数
/// Saves byte data to a file at the specified path.
/// 
/// # Arguments
/// 
/// * `data` - A slice of bytes to be written to the file.
/// * `file_path` - The path where the file will be created and the data will be written.
fn save_to_file(data: &[u8], file_path: &str) -> std::io::Result<()> {
    let mut file = File::create(file_path)?;
    file.write_all(data)?;
    Ok(())
}
