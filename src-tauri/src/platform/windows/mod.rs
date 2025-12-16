// Windows-specific implementations
pub mod apps;

pub use apps::{AppScanner, AppInfo};

/// Extract icon from Windows executable or shortcut
pub async fn extract_app_icon(app_path: &std::path::Path) -> Option<Vec<u8>> {
    let path = app_path.to_path_buf();

    // Icon extraction is blocking (Win32/GDI). Run it off the async runtime.
    tokio::task::spawn_blocking(move || extract_app_icon_sync(&path))
        .await
        .ok()
        .flatten()
}

/// Extract the system icon for an arbitrary file/folder path (Explorer-style).
///
/// This uses the Shell icon for the specific file (or folder), so file search results can display
/// the same icon you see in Windows Explorer.
pub async fn extract_file_icon(path: &std::path::Path) -> Option<Vec<u8>> {
    let path = path.to_path_buf();

    tokio::task::spawn_blocking(move || extract_file_icon_sync(&path))
        .await
        .ok()
        .flatten()
}

fn extract_file_icon_sync(path: &std::path::Path) -> Option<Vec<u8>> {
    use std::ffi::OsStr;
    use std::iter;
    use std::mem::size_of;
    use std::os::windows::ffi::OsStrExt;

    use windows::core::PCWSTR;
    use windows::Win32::Storage::FileSystem::{
        FILE_ATTRIBUTE_DIRECTORY, FILE_ATTRIBUTE_NORMAL, FILE_FLAGS_AND_ATTRIBUTES,
    };
    use windows::Win32::UI::Shell::{
        SHGetFileInfoW, SHFILEINFOW, SHGFI_ICON, SHGFI_LARGEICON, SHGFI_SMALLICON,
        SHGFI_USEFILEATTRIBUTES,
    };
    use windows::Win32::UI::WindowsAndMessaging::DestroyIcon;

    let wide: Vec<u16> = OsStr::new(path.as_os_str())
        .encode_wide()
        .chain(iter::once(0))
        .collect();

    // If the path exists, let Shell decide attributes.
    // If it doesn't (rare for Everything results), fall back to using attributes.
    let (attr, use_attrs) = match std::fs::metadata(path) {
        Ok(md) => {
            if md.is_dir() {
                (FILE_FLAGS_AND_ATTRIBUTES(FILE_ATTRIBUTE_DIRECTORY.0), false)
            } else {
                (FILE_FLAGS_AND_ATTRIBUTES(FILE_ATTRIBUTE_NORMAL.0), false)
            }
        }
        Err(_) => {
            // Guess based on trailing separator/extension.
            let guess_dir = path.as_os_str().to_string_lossy().ends_with('\\');
            if guess_dir {
                (FILE_FLAGS_AND_ATTRIBUTES(FILE_ATTRIBUTE_DIRECTORY.0), true)
            } else {
                (FILE_FLAGS_AND_ATTRIBUTES(FILE_ATTRIBUTE_NORMAL.0), true)
            }
        }
    };

    unsafe {
        let mut info = SHFILEINFOW::default();
        let mut flags_large = SHGFI_ICON | SHGFI_LARGEICON;
        let mut flags_small = SHGFI_ICON | SHGFI_SMALLICON;
        if use_attrs {
            flags_large |= SHGFI_USEFILEATTRIBUTES;
            flags_small |= SHGFI_USEFILEATTRIBUTES;
        }

        let hicon = {
            let r = SHGetFileInfoW(
                PCWSTR(wide.as_ptr()),
                attr,
                Some(&mut info),
                size_of::<SHFILEINFOW>() as u32,
                flags_large,
            );
            if r == 0 || info.hIcon.0.is_null() {
                info = SHFILEINFOW::default();
                let r2 = SHGetFileInfoW(
                    PCWSTR(wide.as_ptr()),
                    attr,
                    Some(&mut info),
                    size_of::<SHFILEINFOW>() as u32,
                    flags_small,
                );
                if r2 == 0 {
                    return None;
                }
            }
            info.hIcon
        };

        if hicon.0.is_null() {
            return None;
        }

        let png = hicon_to_png(hicon);
        let _ = DestroyIcon(hicon);
        png
    }
}

fn extract_app_icon_sync(app_path: &std::path::Path) -> Option<Vec<u8>> {
    use std::ffi::OsStr;
    use std::iter;
    use std::mem::size_of;
    use std::os::windows::ffi::OsStrExt;

    use windows::core::PCWSTR;
    use windows::Win32::Storage::FileSystem::FILE_FLAGS_AND_ATTRIBUTES;
    use windows::Win32::UI::Shell::{SHGetFileInfoW, SHFILEINFOW, SHGFI_ICON, SHGFI_LARGEICON, SHGFI_SMALLICON};
    use windows::Win32::UI::WindowsAndMessaging::DestroyIcon;

    // Only attempt for typical app targets.
    let ext = app_path.extension().and_then(|e| e.to_str()).unwrap_or("").to_ascii_lowercase();
    if ext != "exe" && ext != "lnk" && ext != "ico" {
        return None;
    }

    let wide: Vec<u16> = OsStr::new(app_path.as_os_str())
        .encode_wide()
        .chain(iter::once(0))
        .collect();

    unsafe {
        let mut info = SHFILEINFOW::default();
        let flags_large = SHGFI_ICON | SHGFI_LARGEICON;
        let flags_small = SHGFI_ICON | SHGFI_SMALLICON;

        let hicon = {
            let r = SHGetFileInfoW(
                PCWSTR(wide.as_ptr()),
                FILE_FLAGS_AND_ATTRIBUTES(0),
                Some(&mut info),
                size_of::<SHFILEINFOW>() as u32,
                flags_large,
            );
            if r == 0 || info.hIcon.0.is_null() {
                // fallback to small icon
                info = SHFILEINFOW::default();
                let r2 = SHGetFileInfoW(
                    PCWSTR(wide.as_ptr()),
                    FILE_FLAGS_AND_ATTRIBUTES(0),
                    Some(&mut info),
                    size_of::<SHFILEINFOW>() as u32,
                    flags_small,
                );
                if r2 == 0 {
                    return None;
                }
            }
            info.hIcon
        };

        if hicon.0.is_null() {
            return None;
        }

        let png = hicon_to_png(hicon);
        let _ = DestroyIcon(hicon);
        png
    }
}

unsafe fn hicon_to_png(hicon: windows::Win32::UI::WindowsAndMessaging::HICON) -> Option<Vec<u8>> {
    use std::io::Cursor;

    use windows::Win32::Graphics::Gdi::{
        CreateCompatibleDC, DeleteDC, DeleteObject, GetDIBits, GetObjectW, BITMAP, BITMAPINFO,
        BITMAPINFOHEADER, BI_RGB, DIB_RGB_COLORS,
    };
    use windows::Win32::UI::WindowsAndMessaging::{GetIconInfo, ICONINFO};

    let mut icon_info = ICONINFO::default();
    if GetIconInfo(hicon, &mut icon_info).is_err() {
        return None;
    }

    let hbm_color = icon_info.hbmColor;
    let hbm_mask = icon_info.hbmMask;

    if hbm_color.0.is_null() {
        if !hbm_mask.0.is_null() {
            let _ = DeleteObject(hbm_mask);
        }
        return None;
    }

    let mut bmp = BITMAP::default();
    if GetObjectW(
        hbm_color,
        std::mem::size_of::<BITMAP>() as i32,
        Some((&mut bmp) as *mut _ as *mut _),
    ) == 0
    {
        let _ = DeleteObject(hbm_color);
        if !hbm_mask.0.is_null() {
            let _ = DeleteObject(hbm_mask);
        }
        return None;
    }

    let width = bmp.bmWidth.max(1) as u32;
    let height = bmp.bmHeight.max(1) as u32;

    let mut bmi = BITMAPINFO {
        bmiHeader: BITMAPINFOHEADER {
            biSize: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
            biWidth: width as i32,
            biHeight: -(height as i32),
            biPlanes: 1,
            biBitCount: 32,
            biCompression: BI_RGB.0 as u32,
            biSizeImage: 0,
            biXPelsPerMeter: 0,
            biYPelsPerMeter: 0,
            biClrUsed: 0,
            biClrImportant: 0,
        },
        bmiColors: [Default::default(); 1],
    };

    let mut bgra = vec![0u8; (width * height * 4) as usize];
    let hdc = CreateCompatibleDC(None);
    if hdc.0.is_null() {
        let _ = DeleteObject(hbm_color);
        if !hbm_mask.0.is_null() {
            let _ = DeleteObject(hbm_mask);
        }
        return None;
    }

    let scanlines = GetDIBits(
        hdc,
        hbm_color,
        0,
        height,
        Some(bgra.as_mut_ptr() as *mut _),
        &mut bmi,
        DIB_RGB_COLORS,
    );
    let _ = DeleteDC(hdc);

    // Always delete the bitmaps returned by GetIconInfo
    let _ = DeleteObject(hbm_color);

    if scanlines == 0 {
        if !hbm_mask.0.is_null() {
            let _ = DeleteObject(hbm_mask);
        }
        return None;
    }

    // If alpha is empty, try to compute from mask.
    let mut alpha_all_zero = true;
    for px in bgra.chunks_exact(4) {
        if px[3] != 0 {
            alpha_all_zero = false;
            break;
        }
    }

    let mut alpha_from_mask: Option<Vec<u8>> = None;
    if alpha_all_zero && !hbm_mask.0.is_null() {
        let mut mask_bmp = BITMAP::default();
        if GetObjectW(
            hbm_mask,
            std::mem::size_of::<BITMAP>() as i32,
            Some((&mut mask_bmp) as *mut _ as *mut _),
        ) != 0
        {
            let mask_width = mask_bmp.bmWidth.max(1) as u32;
            let mask_height = mask_bmp.bmHeight.max(1) as u32;
            let effective_h = if mask_height == height * 2 { height } else { height };

            let mut mask_bmi = BITMAPINFO {
                bmiHeader: BITMAPINFOHEADER {
                    biSize: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
                    biWidth: mask_width as i32,
                    biHeight: -(effective_h as i32),
                    biPlanes: 1,
                    biBitCount: 32,
                    biCompression: BI_RGB.0 as u32,
                    biSizeImage: 0,
                    biXPelsPerMeter: 0,
                    biYPelsPerMeter: 0,
                    biClrUsed: 0,
                    biClrImportant: 0,
                },
                bmiColors: [Default::default(); 1],
            };

            let mut mask_rgba = vec![0u8; (mask_width * effective_h * 4) as usize];
            let hdc2 = CreateCompatibleDC(None);
            if !hdc2.0.is_null() {
                let _ = GetDIBits(
                    hdc2,
                    hbm_mask,
                    0,
                    effective_h,
                    Some(mask_rgba.as_mut_ptr() as *mut _),
                    &mut mask_bmi,
                    DIB_RGB_COLORS,
                );
                let _ = DeleteDC(hdc2);

                let mut a = vec![255u8; (width * height) as usize];
                for y in 0..height {
                    for x in 0..width {
                        let i = (y * width + x) as usize;
                        let mi = (y * mask_width + x) as usize * 4;
                        if mi + 3 < mask_rgba.len() {
                            let v = mask_rgba[mi];
                            a[i] = if v > 127 { 0 } else { 255 };
                        }
                    }
                }
                alpha_from_mask = Some(a);
            }
        }
    }

    if !hbm_mask.0.is_null() {
        let _ = DeleteObject(hbm_mask);
    }

    let mut rgba = vec![0u8; bgra.len()];
    for (i, px) in bgra.chunks_exact(4).enumerate() {
        let o = i * 4;
        rgba[o] = px[2];
        rgba[o + 1] = px[1];
        rgba[o + 2] = px[0];
        rgba[o + 3] = alpha_from_mask.as_ref().map(|a| a[i]).unwrap_or(px[3]);
    }

    let img = image::RgbaImage::from_raw(width, height, rgba)?;
    let mut out = Vec::new();
    let mut cur = Cursor::new(&mut out);
    image::DynamicImage::ImageRgba8(img)
        .write_to(&mut cur, image::ImageOutputFormat::Png)
        .ok()?;
    Some(out)
}

/// Launch an application
pub async fn launch_app(app_path: &std::path::Path) -> Result<(), String> {
    use tokio::process::Command;

    // For .lnk files, use cmd /c start
    // For .exe files, can run directly
    if let Some(ext) = app_path.extension() {
        if ext == "lnk" {
            let output = Command::new("cmd")
                .args(&["/c", "start", "", app_path.to_string_lossy().as_ref()])
                .output()
                .await
                .map_err(|e| format!("Failed to launch app: {}", e))?;

            if output.status.success() {
                return Ok(());
            }
        } else if ext == "exe" {
            let output = Command::new(app_path)
                .output()
                .await
                .map_err(|e| format!("Failed to launch app: {}", e))?;

            if output.status.success() {
                return Ok(());
            }
        }
    }

    Err("Unsupported file type".to_string())
}

