use base64::engine::general_purpose::STANDARD as BASE64;
use base64::Engine;

/// Windows 10/11 native OCR (WinRT).
///
/// Accepts either raw base64 or a full data URL (`data:image/png;base64,...`).
#[tauri::command]
pub async fn recognize_text(base64_image: String) -> Result<String, String> {
    recognize_text_impl(base64_image).await
}

#[cfg(windows)]
async fn recognize_text_impl(base64_image: String) -> Result<String, String> {
    // WinRT async ops in windows 0.58 are easiest to run synchronously via .get().
    // Wrap in spawn_blocking to avoid blocking the async runtime thread.
    tauri::async_runtime::spawn_blocking(move || recognize_text_sync(base64_image))
        .await
        .map_err(|e| format!("OCR task join failed: {e}"))?
}

#[cfg(windows)]
fn recognize_text_sync(base64_image: String) -> Result<String, String> {
    use windows::Graphics::Imaging::{BitmapAlphaMode, BitmapDecoder, BitmapPixelFormat, SoftwareBitmap};
    use windows::Media::Ocr::OcrEngine;
    use windows::Globalization::{ApplicationLanguages, Language};
    use windows::Storage::Streams::{DataWriter, InMemoryRandomAccessStream};
    use windows::Win32::Foundation::RPC_E_CHANGED_MODE;
    use windows::Win32::System::Com::{CoInitializeEx, COINIT_MULTITHREADED};
    use windows::core::HSTRING;

    // Best-effort COM initialization for WinRT usage.
    // If the process is already initialized with a different apartment model, ignore.
    unsafe {
        let hr = CoInitializeEx(None, COINIT_MULTITHREADED);
        if hr.is_err() && hr != RPC_E_CHANGED_MODE {
            return Err(format!("CoInitializeEx failed: {hr:?}"));
        }
    }

    // 1) Decode base64
    let b64 = base64_image
        .split(',')
        .last()
        .unwrap_or(base64_image.as_str())
        .trim();

    let bytes = BASE64
        .decode(b64)
        .map_err(|e| format!("Base64 decode failed: {e}"))?;

    // 2) Bitmap conversion via BitmapDecoder from an in-memory stream
    let mem = InMemoryRandomAccessStream::new()
        .map_err(|e| format!("Create stream failed: {e:?}"))?;

    let writer = DataWriter::CreateDataWriter(&mem)
        .map_err(|e| format!("Create DataWriter failed: {e:?}"))?;

    writer
        .WriteBytes(&bytes)
        .map_err(|e| format!("WriteBytes failed: {e:?}"))?;

    writer
        .StoreAsync()
        .map_err(|e| format!("StoreAsync failed: {e:?}"))?
        .get()
        .map_err(|e| format!("StoreAsync.get failed: {e:?}"))?;

    writer
        .FlushAsync()
        .map_err(|e| format!("FlushAsync failed: {e:?}"))?
        .get()
        .map_err(|e| format!("FlushAsync.get failed: {e:?}"))?;

    mem.Seek(0)
        .map_err(|e| format!("Stream seek failed: {e:?}"))?;

    let decoder = BitmapDecoder::CreateAsync(&mem)
        .map_err(|e| format!("BitmapDecoder::CreateAsync failed: {e:?}"))?
        .get()
        .map_err(|e| format!("BitmapDecoder::CreateAsync.get failed: {e:?}"))?;

    let mut bitmap = decoder
        .GetSoftwareBitmapAsync()
        .map_err(|e| format!("GetSoftwareBitmapAsync failed: {e:?}"))?
        .get()
        .map_err(|e| format!("GetSoftwareBitmapAsync.get failed: {e:?}"))?;

    // Critical: OcrEngine typically requires BGRA8 + Premultiplied.
    let pixel_format = bitmap
        .BitmapPixelFormat()
        .map_err(|e| format!("BitmapPixelFormat failed: {e:?}"))?;
    let alpha_mode = bitmap
        .BitmapAlphaMode()
        .map_err(|e| format!("BitmapAlphaMode failed: {e:?}"))?;

    if pixel_format != BitmapPixelFormat::Bgra8 || alpha_mode != BitmapAlphaMode::Premultiplied {
        bitmap = SoftwareBitmap::ConvertWithAlpha(&bitmap, BitmapPixelFormat::Bgra8, BitmapAlphaMode::Premultiplied)
            .map_err(|e| format!("SoftwareBitmap::ConvertWithAlpha failed: {e:?}"))?;
    }

    let run_with_engine = |engine: &OcrEngine| -> Result<String, String> {
        let result = engine
            .RecognizeAsync(&bitmap)
            .map_err(|e| format!("RecognizeAsync failed: {e:?}"))?
            .get()
            .map_err(|e| format!("RecognizeAsync.get failed: {e:?}"))?;

        let lines = result
            .Lines()
            .map_err(|e| format!("Result.Lines failed: {e:?}"))?;

        let mut out = String::new();
        let count = lines
            .Size()
            .map_err(|e| format!("Lines.Size failed: {e:?}"))?;

        for i in 0..count {
            let line = lines
                .GetAt(i)
                .map_err(|e| format!("Lines.GetAt({i}) failed: {e:?}"))?;
            let text = line
                .Text()
                .map_err(|e| format!("Line.Text failed: {e:?}"))?;
            if !out.is_empty() {
                out.push('\n');
            }
            out.push_str(&text.to_string());
        }

        Ok(out.trim().to_string())
    };

    // 3) Recognize
    // Strategy:
    // - First: user profile language engine
    // - If empty: try common languages (English/Chinese) if available
    // - Then: try user preferred language tags (ApplicationLanguages)
    let engine = OcrEngine::TryCreateFromUserProfileLanguages()
        .map_err(|e| format!("TryCreateFromUserProfileLanguages failed: {e:?}"))?;

    let first = run_with_engine(&engine)?;
    if !first.is_empty() {
        return Ok(first);
    }

    let mut available_tags: Vec<String> = Vec::new();
    if let Ok(langs) = OcrEngine::AvailableRecognizerLanguages() {
        if let Ok(size) = langs.Size() {
            for i in 0..size {
                if let Ok(lang) = langs.GetAt(i) {
                    if let Ok(tag) = lang.LanguageTag() {
                        available_tags.push(tag.to_string());
                    }
                }
            }
        }
    }

    let find_available_tag = |wanted: &str| -> Option<String> {
        let w = wanted.to_ascii_lowercase();
        available_tags
            .iter()
            .find(|t| t.to_ascii_lowercase() == w)
            .or_else(|| available_tags.iter().find(|t| t.to_ascii_lowercase().starts_with(&(w.clone() + "-"))))
            .map(|t| t.to_string())
    };

    let mut candidates: Vec<String> = Vec::new();
    // Common targets
    for t in ["en-US", "en", "zh-Hans", "zh-CN", "zh"] {
        candidates.push(t.to_string());
    }
    // Also include user preferred tags
    if let Ok(tags) = ApplicationLanguages::Languages() {
        if let Ok(size) = tags.Size() {
            for i in 0..size {
                if let Ok(tag) = tags.GetAt(i) {
                    candidates.push(tag.to_string());
                }
            }
        }
    }

    // Try candidates
    for wanted in candidates {
        let Some(actual) = find_available_tag(&wanted) else { continue };
        let lang = Language::CreateLanguage(&HSTRING::from(actual.clone()))
            .map_err(|e| format!("CreateLanguage({actual}) failed: {e:?}"))?;
        let eng = OcrEngine::TryCreateFromLanguage(&lang)
            .map_err(|e| format!("TryCreateFromLanguage({actual}) failed: {e:?}"))?;
        let text = run_with_engine(&eng)?;
        if !text.is_empty() {
            return Ok(text);
        }
    }

    Ok(String::new())
}

#[cfg(not(windows))]
async fn recognize_text_impl(_base64_image: String) -> Result<String, String> {
    Err("OCR is only supported on Windows".to_string())
}
