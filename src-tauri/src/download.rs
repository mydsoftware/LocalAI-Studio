use reqwest::blocking::Client;
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Serialize)]
pub struct DownloadResult {
    pub path: String,
    pub bytes: u64,
    pub sha256: String,
    pub resumed: bool,
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
    let existing = fs::metadata(&temp).map(|m| m.len()).unwrap_or(0);
    let client = Client::builder()
        .timeout(Duration::from_secs(3600))
        .user_agent("LocalAI-Studio/1.0")
        .build()
        .map_err(|e| format!("ساخت کلاینت دانلود ناموفق بود: {e}"))?;
    let started = Instant::now();
    let mut request = client.get(url);
    if existing > 0 {
        request = request.header(reqwest::header::RANGE, format!("bytes={existing}-"));
    }
    let mut response = request
        .send()
        .and_then(|r| r.error_for_status())
        .map_err(|e| format!("دانلود ناموفق بود: {e}"))?;
    let resumed = existing > 0 && response.status() == reqwest::StatusCode::PARTIAL_CONTENT;
    let mut file = if resumed {
        let mut f = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&temp)
            .map_err(|e| format!("بازکردن فایل ادامه دانلود ناموفق بود: {e}"))?;
        f.seek(SeekFrom::End(0))
            .map_err(|e| format!("تنظیم موقعیت فایل ناموفق بود: {e}"))?;
        f
    } else {
        File::create(&temp).map_err(|e| format!("ساخت فایل موقت ناموفق بود: {e}"))?
    };
    let mut hasher = Sha256::new();
    if resumed {
        let mut prefix = File::open(&temp)
            .map_err(|e| format!("خواندن فایل ادامه دانلود ناموفق بود: {e}"))?;
        let mut buffer = [0u8; 1024 * 1024];
        loop {
            let read = prefix
                .read(&mut buffer)
                .map_err(|e| format!("محاسبه SHA-256 بخش قبلی ناموفق بود: {e}"))?;
            if read == 0 {
                break;
            }
            hasher.update(&buffer[..read]);
        }
    }
    let mut total = if resumed { existing } else { 0 };
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
            return Err(format!(
                "اعتبارسنجی SHA-256 شکست خورد؛ مقدار دریافت‌شده: {sha256}"
            ));
        }
    }
    fs::rename(&temp, &destination).map_err(|e| format!("ثبت فایل نهایی ناموفق بود: {e}"))?;
    let _elapsed = started.elapsed();
    Ok(DownloadResult {
        path: destination.display().to_string(),
        bytes: total,
        sha256,
        resumed,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_unsupported_scheme() {
        let result = super::download(
            "ftp://example.com/model.gguf",
            Path::new("target/test-model.gguf"),
            None,
        );
        assert!(result.is_err());
    }

    #[test]
    fn creates_nested_destination_parent() {
        let root = std::env::temp_dir().join(format!(
            "localai-studio-test-{}",
            std::process::id()
        ));
        let path = root.join("nested/model.gguf");
        let result = safe_destination(&path);
        assert!(result.is_ok());
        assert!(path.parent().is_some_and(Path::exists));
        let _ = fs::remove_dir_all(root);
    }
}
