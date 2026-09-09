use reqwest::blocking::Client;
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::time::Duration;

#[derive(Debug, Clone, Serialize)]
pub struct DownloadResult {
    pub path: String,
    pub bytes: u64,
    pub sha256: String,
}

fn safe_destination(destination: &Path) -> Result<PathBuf, String> {
    if destination.as_os_str().is_empty() {
        return Err("مسیر مقصد دانلود خالی است.".into());
    }
    let parent = destination
        .parent()
        .ok_or_else(|| "مسیر مقصد نامعتبر است.".to_string())?;
    fs::create_dir_all(parent).map_err(|e| format!("ساخت پوشه مقصد ناموفق بود: {e}"))?;
    Ok(destination.to_path_buf())
}

pub fn download(
    url: &str,
    destination: &Path,
    expected_sha256: Option<&str>,
) -> Result<DownloadResult, String> {
    if !(url.starts_with("https://") || url.starts_with("http://")) {
        return Err("فقط URLهای HTTP/HTTPS مجاز هستند.".into());
    }
    let destination = safe_destination(destination)?;
    let temp = destination.with_extension("part");
    let client = Client::builder()
        .timeout(Duration::from_secs(3600))
        .build()
        .map_err(|e| format!("ساخت کلاینت دانلود ناموفق بود: {e}"))?;
    let mut response = client
        .get(url)
        .send()
        .and_then(|r| r.error_for_status())
        .map_err(|e| format!("دانلود ناموفق بود: {e}"))?;

    let mut file = File::create(&temp)
        .map_err(|e| format!("ساخت فایل موقت ناموفق بود: {e}"))?;
    let mut hasher = Sha256::new();
    let mut total = 0u64;
    let mut buffer = [0u8; 1024 * 1024];
    loop {
        let read = response
            .read(&mut buffer)
            .map_err(|e| format!("خواندن دانلود ناموفق بود: {e}"))?;
        if read == 0 {
            break;
        }
        file.write_all(&buffer[..read])
            .map_err(|e| format!("نوشتن فایل مدل ناموفق بود: {e}"))?;
        hasher.update(&buffer[..read]);
        total += read as u64;
    }
    file.flush()
        .map_err(|e| format!("ثبت فایل دانلود ناموفق بود: {e}"))?;
    let sha256 = format!("{:x}", hasher.finalize());
    if let Some(expected) = expected_sha256 {
        if !expected.eq_ignore_ascii_case(&sha256) {
            let _ = fs::remove_file(&temp);
            return Err(format!(
                "اعتبارسنجی SHA-256 شکست خورد؛ مقدار دریافت‌شده: {sha256}"
            ));
        }
    }
    fs::rename(&temp, &destination)
        .map_err(|e| format!("ثبت فایل نهایی ناموفق بود: {e}"))?;
    Ok(DownloadResult {
        path: destination.display().to_string(),
        bytes: total,
        sha256,
    })
}
