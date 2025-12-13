//! Everything Service - Enhanced File Search
//! 
//! Provides file search via Everything64.dll with:
//! - Wildcard injection for fuzzy matching
//! - Case-insensitive search
//! - Smart query preprocessing

use std::path::PathBuf;
use std::os::raw::{c_int, c_uint};
use std::sync::OnceLock;

use libloading::Library;
use serde::Serialize;
use tauri::Manager;

// ═══════════════════════════════════════════════════════════════════════════════
// FFI Type Definitions
// ═══════════════════════════════════════════════════════════════════════════════

type EverythingSetSearchW = unsafe extern "system" fn(*const u16);
type EverythingSetRequestFlags = unsafe extern "system" fn(c_uint);
type EverythingSetMax = unsafe extern "system" fn(c_uint);
type EverythingSetMatchCase = unsafe extern "system" fn(c_int);
type EverythingSetMatchWholeWord = unsafe extern "system" fn(c_int);
type EverythingSetMatchPath = unsafe extern "system" fn(c_int);
type EverythingSetSort = unsafe extern "system" fn(c_uint);
type EverythingQueryW = unsafe extern "system" fn(c_int) -> c_int;
type EverythingGetNumResults = unsafe extern "system" fn() -> c_uint;
type EverythingGetResultFullPathNameW = unsafe extern "system" fn(c_uint, *mut u16, c_uint) -> c_uint;
type EverythingGetLastError = unsafe extern "system" fn() -> c_uint;
type EverythingReset = unsafe extern "system" fn();
type EverythingCleanUp = unsafe extern "system" fn();
type EverythingGetResultSize = unsafe extern "system" fn(c_uint, *mut i64) -> c_int;
type EverythingGetResultDateModified = unsafe extern "system" fn(c_uint, *mut i64) -> c_int;

// Request flags
const EVERYTHING_REQUEST_FULL_PATH_AND_FILE_NAME: c_uint = 0x00000004;
const EVERYTHING_REQUEST_SIZE: c_uint = 0x00000010;
const EVERYTHING_REQUEST_DATE_MODIFIED: c_uint = 0x00000040;

// Sort options
const EVERYTHING_SORT_NAME_ASCENDING: c_uint = 1;
const EVERYTHING_SORT_DATE_MODIFIED_DESCENDING: c_uint = 12;

// Error codes
const EVERYTHING_OK: c_uint = 0;
const EVERYTHING_ERROR_MEMORY: c_uint = 1;
const EVERYTHING_ERROR_IPC: c_uint = 2;
const EVERYTHING_ERROR_REGISTERCLASSEX: c_uint = 3;
const EVERYTHING_ERROR_CREATEWINDOW: c_uint = 4;
const EVERYTHING_ERROR_CREATETHREAD: c_uint = 5;
const EVERYTHING_ERROR_INVALIDINDEX: c_uint = 6;
const EVERYTHING_ERROR_INVALIDCALL: c_uint = 7;

// ═══════════════════════════════════════════════════════════════════════════════
// Everything Library Wrapper
// ═══════════════════════════════════════════════════════════════════════════════

struct EverythingLib {
    _lib: Library,
    set_search_w: EverythingSetSearchW,
    set_request_flags: EverythingSetRequestFlags,
    set_max: EverythingSetMax,
    set_match_case: EverythingSetMatchCase,
    set_match_whole_word: EverythingSetMatchWholeWord,
    set_match_path: EverythingSetMatchPath,
    set_sort: EverythingSetSort,
    query_w: EverythingQueryW,
    get_num_results: EverythingGetNumResults,
    get_result_full_path_name_w: EverythingGetResultFullPathNameW,
    get_last_error: EverythingGetLastError,
    reset: EverythingReset,
    cleanup: EverythingCleanUp,
    get_result_size: EverythingGetResultSize,
    get_result_date_modified: EverythingGetResultDateModified,
}

unsafe impl Send for EverythingLib {}
unsafe impl Sync for EverythingLib {}

impl EverythingLib {
    fn new(dll_path: &PathBuf) -> Result<Self, String> {
        unsafe {
            let lib = Library::new(dll_path)
                .map_err(|e| format!("Failed to load Everything64.dll: {}", e))?;
            
            let set_search_w = *lib
                .get::<EverythingSetSearchW>(b"Everything_SetSearchW")
                .map_err(|e| format!("Failed to get Everything_SetSearchW: {}", e))?;
            
            let set_request_flags = *lib
                .get::<EverythingSetRequestFlags>(b"Everything_SetRequestFlags")
                .map_err(|e| format!("Failed to get Everything_SetRequestFlags: {}", e))?;
            
            let set_max = *lib
                .get::<EverythingSetMax>(b"Everything_SetMax")
                .map_err(|e| format!("Failed to get Everything_SetMax: {}", e))?;
            
            let set_match_case = *lib
                .get::<EverythingSetMatchCase>(b"Everything_SetMatchCase")
                .map_err(|e| format!("Failed to get Everything_SetMatchCase: {}", e))?;
            
            let set_match_whole_word = *lib
                .get::<EverythingSetMatchWholeWord>(b"Everything_SetMatchWholeWord")
                .map_err(|e| format!("Failed to get Everything_SetMatchWholeWord: {}", e))?;
            
            let set_match_path = *lib
                .get::<EverythingSetMatchPath>(b"Everything_SetMatchPath")
                .map_err(|e| format!("Failed to get Everything_SetMatchPath: {}", e))?;
            
            let set_sort = *lib
                .get::<EverythingSetSort>(b"Everything_SetSort")
                .map_err(|e| format!("Failed to get Everything_SetSort: {}", e))?;
            
            let query_w = *lib
                .get::<EverythingQueryW>(b"Everything_QueryW")
                .map_err(|e| format!("Failed to get Everything_QueryW: {}", e))?;
            
            let get_num_results = *lib
                .get::<EverythingGetNumResults>(b"Everything_GetNumResults")
                .map_err(|e| format!("Failed to get Everything_GetNumResults: {}", e))?;
            
            let get_result_full_path_name_w = *lib
                .get::<EverythingGetResultFullPathNameW>(b"Everything_GetResultFullPathNameW")
                .map_err(|e| format!("Failed to get Everything_GetResultFullPathNameW: {}", e))?;
            
            let get_last_error = *lib
                .get::<EverythingGetLastError>(b"Everything_GetLastError")
                .map_err(|e| format!("Failed to get Everything_GetLastError: {}", e))?;
            
            let reset = *lib
                .get::<EverythingReset>(b"Everything_Reset")
                .map_err(|e| format!("Failed to get Everything_Reset: {}", e))?;
            
            let cleanup = *lib
                .get::<EverythingCleanUp>(b"Everything_CleanUp")
                .map_err(|e| format!("Failed to get Everything_CleanUp: {}", e))?;
            
            let get_result_size = *lib
                .get::<EverythingGetResultSize>(b"Everything_GetResultSize")
                .map_err(|e| format!("Failed to get Everything_GetResultSize: {}", e))?;
            
            let get_result_date_modified = *lib
                .get::<EverythingGetResultDateModified>(b"Everything_GetResultDateModified")
                .map_err(|e| format!("Failed to get Everything_GetResultDateModified: {}", e))?;
            
            Ok(Self {
                _lib: lib,
                set_search_w,
                set_request_flags,
                set_max,
                set_match_case,
                set_match_whole_word,
                set_match_path,
                set_sort,
                query_w,
                get_num_results,
                get_result_full_path_name_w,
                get_last_error,
                reset,
                cleanup,
                get_result_size,
                get_result_date_modified,
            })
        }
    }
    
    fn search(&self, query: &str, max_results: u32) -> Result<Vec<FileSearchResult>, String> {
        unsafe {
            // Reset state
            (self.reset)();
            
            // Convert query to wide string
            let query_wide: Vec<u16> = query.encode_utf16().chain(std::iter::once(0)).collect();
            
            // Configure search options
            (self.set_match_case)(0);        // Case insensitive
            (self.set_match_whole_word)(0);  // Partial match
            (self.set_match_path)(0);        // Match filename only, not full path
            (self.set_sort)(EVERYTHING_SORT_DATE_MODIFIED_DESCENDING); // Recent files first
            
            // Set search parameters
            (self.set_search_w)(query_wide.as_ptr());
            (self.set_request_flags)(
                EVERYTHING_REQUEST_FULL_PATH_AND_FILE_NAME |
                EVERYTHING_REQUEST_SIZE |
                EVERYTHING_REQUEST_DATE_MODIFIED
            );
            (self.set_max)(max_results);
            
            // Execute query (1 = wait for results)
            let success = (self.query_w)(1);
            
            if success == 0 {
                let error = (self.get_last_error)();
                return Err(self.error_to_string(error));
            }
            
            // Get results
            let num_results = (self.get_num_results)();
            let mut results = Vec::with_capacity(num_results as usize);
            
            for i in 0..num_results {
                // Get path
                let mut path_buf: Vec<u16> = vec![0; 1024];
                let len = (self.get_result_full_path_name_w)(i, path_buf.as_mut_ptr(), 1024);
                
                if len > 0 {
                    path_buf.truncate(len as usize);
                    let path = String::from_utf16_lossy(&path_buf);
                    
                    // Get size
                    let mut size: i64 = 0;
                    (self.get_result_size)(i, &mut size);
                    
                    // Get modification date
                    let mut date_modified: i64 = 0;
                    (self.get_result_date_modified)(i, &mut date_modified);
                    
                    // Extract filename
                    let file_path = std::path::Path::new(&path);
                    let filename = file_path.file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("")
                        .to_string();
                    
                    // Determine if it's a folder
                    let is_folder = file_path.extension().is_none() && size == 0;
                    
                    // Get extension
                    let extension = file_path.extension()
                        .and_then(|e| e.to_str())
                        .map(|s| s.to_lowercase())
                        .unwrap_or_default();
                    
                    // Smart classification
                    let category = classify_file(&path, &extension);
                    
                    // Generate clean display path
                    let display_path = get_display_path(&path, &filename);
                    
                    results.push(FileSearchResult {
                        path,
                        filename,
                        extension,
                        size: if size > 0 { Some(size as u64) } else { None },
                        date_modified: if date_modified > 0 { Some(date_modified) } else { None },
                        is_folder,
                        category,
                        display_path,
                    });
                }
            }
            
            Ok(results)
        }
    }
    
    fn error_to_string(&self, error: c_uint) -> String {
        match error {
            EVERYTHING_OK => "OK".to_string(),
            EVERYTHING_ERROR_MEMORY => "Memory allocation error".to_string(),
            EVERYTHING_ERROR_IPC => "Everything IPC error - is Everything running?".to_string(),
            EVERYTHING_ERROR_REGISTERCLASSEX => "Failed to register window class".to_string(),
            EVERYTHING_ERROR_CREATEWINDOW => "Failed to create window".to_string(),
            EVERYTHING_ERROR_CREATETHREAD => "Failed to create thread".to_string(),
            EVERYTHING_ERROR_INVALIDINDEX => "Invalid index".to_string(),
            EVERYTHING_ERROR_INVALIDCALL => "Invalid call".to_string(),
            _ => format!("Unknown error: {}", error),
        }
    }
}

impl Drop for EverythingLib {
    fn drop(&mut self) {
        unsafe {
            (self.cleanup)();
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Global State
// ═══════════════════════════════════════════════════════════════════════════════

static EVERYTHING: OnceLock<Result<EverythingLib, String>> = OnceLock::new();

// ═══════════════════════════════════════════════════════════════════════════════
// Public API
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Serialize, Clone)]
pub struct FileSearchResult {
    pub path: String,
    pub filename: String,
    pub extension: String,
    pub size: Option<u64>,
    pub date_modified: Option<i64>,
    pub is_folder: bool,
    /// Smart category: "Application" or "File"
    pub category: String,
    /// Clean display path (resolves Recent shortcuts)
    pub display_path: String,
}

// ═══════════════════════════════════════════════════════════════════════════════
// Smart Classification
// ═══════════════════════════════════════════════════════════════════════════════

/// Classify a file based on its path and extension
/// 
/// Rules:
/// - .exe files are always Applications
/// - .lnk files:
///   - In "Recent" folder -> File (history shortcut, not app)
///   - In "Start Menu" or "Desktop" -> Application
///   - Otherwise -> File
/// - Everything else -> File
fn classify_file(path: &str, extension: &str) -> String {
    let path_lower = path.to_lowercase();
    let ext_lower = extension.to_lowercase();
    
    match ext_lower.as_str() {
        "exe" => "Application".to_string(),
        "lnk" => {
            // Check for Recent folder - these are NOT apps
            if path_lower.contains("\\recent\\")
                || path_lower.contains("microsoft\\windows\\recent")
                || path_lower.contains("/recent/")
            {
                return "File".to_string();
            }
            
            // Check for legitimate app locations
            if path_lower.contains("start menu")
                || path_lower.contains("\\desktop\\")
                || path_lower.contains("/desktop/")
                || path_lower.ends_with("\\desktop")
                || path_lower.contains("\\programs\\")
            {
                return "Application".to_string();
            }
            
            // Default: treat as file
            "File".to_string()
        }
        "msi" => "Application".to_string(),
        _ => "File".to_string(),
    }
}

/// Generate a clean display path
/// For Recent folder shortcuts, show the original filename without the confusing path
fn get_display_path(path: &str, filename: &str) -> String {
    let path_lower = path.to_lowercase();
    
    // For Recent folder items, just show the filename (it's more meaningful)
    if path_lower.contains("\\recent\\")
        || path_lower.contains("microsoft\\windows\\recent")
    {
        return format!("Recent: {}", filename);
    }
    
    // For very long paths, try to shorten them
    if path.len() > 80 {
        // Try to show just the last 2-3 path components
        let parts: Vec<&str> = path.split('\\').collect();
        if parts.len() > 3 {
            return format!("...\\{}", parts[parts.len()-3..].join("\\"));
        }
    }
    
    path.to_string()
}

/// Initialize Everything DLL
pub fn init_everything(app_handle: &tauri::AppHandle) -> Result<(), String> {
    let dll_path = match resolve_dll_path(app_handle) {
        Ok(path) => path,
        Err(e) => {
            tracing::error!("Failed to resolve Everything64.dll path: {}", e);
            return Err(e);
        }
    };
    
    tracing::info!("Loading Everything64.dll from: {:?}", dll_path);
    
    EVERYTHING.get_or_init(|| {
        EverythingLib::new(&dll_path)
    });
    
    match EVERYTHING.get() {
        Some(Ok(_)) => {
            tracing::info!("Everything64.dll loaded successfully");
            Ok(())
        }
        Some(Err(e)) => Err(e.clone()),
        None => Err("Failed to initialize Everything".to_string()),
    }
}

/// Resolve DLL path for both dev and production
fn resolve_dll_path(app_handle: &tauri::AppHandle) -> Result<PathBuf, String> {
    // 1. Try exe directory directly (for portable deployment: exe + Everything64.dll in same folder)
    if let Ok(exe_path) = std::env::current_exe() {
        if let Some(exe_dir) = exe_path.parent() {
            let dll_path = exe_dir.join("Everything64.dll");
            if dll_path.exists() {
                tracing::info!("Found Everything64.dll in exe directory: {:?}", dll_path);
                return Ok(dll_path);
            }
            
            // Also try libs subfolder in exe directory
            let dll_path = exe_dir.join("libs").join("Everything64.dll");
            if dll_path.exists() {
                tracing::info!("Found Everything64.dll in exe/libs: {:?}", dll_path);
                return Ok(dll_path);
            }
        }
    }
    
    // 2. Try resource path (Tauri bundled resources)
    if let Ok(resource_path) = app_handle.path().resource_dir() {
        let dll_path = resource_path.join("libs").join("Everything64.dll");
        if dll_path.exists() {
            tracing::info!("Found Everything64.dll in resource dir: {:?}", dll_path);
            return Ok(dll_path);
        }
        
        // Try directly in resource dir
        let dll_path = resource_path.join("Everything64.dll");
        if dll_path.exists() {
            tracing::info!("Found Everything64.dll in resource root: {:?}", dll_path);
            return Ok(dll_path);
        }
    }
    
    // 3. Try src-tauri/libs (development)
    let dev_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("libs")
        .join("Everything64.dll");
    if dev_path.exists() {
        tracing::info!("Found Everything64.dll in dev path: {:?}", dev_path);
        return Ok(dev_path);
    }
    
    // 4. Try current working directory
    if let Ok(cwd) = std::env::current_dir() {
        let dll_path = cwd.join("Everything64.dll");
        if dll_path.exists() {
            tracing::info!("Found Everything64.dll in cwd: {:?}", dll_path);
            return Ok(dll_path);
        }
        
        let dll_path = cwd.join("libs").join("Everything64.dll");
        if dll_path.exists() {
            tracing::info!("Found Everything64.dll in cwd/libs: {:?}", dll_path);
            return Ok(dll_path);
        }
    }
    
    Err("Everything64.dll not found in any of: exe directory, libs/, resources/, or current directory".to_string())
}

// ═══════════════════════════════════════════════════════════════════════════════
// Smart Query Building
// ═══════════════════════════════════════════════════════════════════════════════

/// Build smart query with wildcards
/// 
/// Transforms user input into Everything-compatible query:
/// - "chrome" -> "*chrome*"
/// - "pkg_bsaml.pck" -> "*pkg_bsaml.pck*" (preserves dots in filenames)
/// - "google chrome" -> "*google* *chrome*" (AND search for multiple words)
fn build_smart_query(input: &str) -> String {
    let input = input.trim();
    
    if input.is_empty() {
        return String::new();
    }
    
    // Check if input looks like a filename (contains a dot followed by extension)
    // Don't split on dots if it looks like "filename.ext"
    let has_extension = input.contains('.') && {
        let parts: Vec<&str> = input.rsplitn(2, '.').collect();
        parts.len() == 2 && parts[0].len() <= 10 && !parts[0].contains(' ')
    };
    
    if has_extension {
        // Treat as a single filename - wrap entire input
        format!("*{}*", input)
    } else if input.contains(' ') {
        // Multiple words - create AND query with wildcards on each word
        input.split_whitespace()
            .map(|word| format!("*{}*", word))
            .collect::<Vec<_>>()
            .join(" ")
    } else {
        // Single word without extension
        format!("*{}*", input)
    }
}

/// Search files using Everything
pub async fn search_files(query: String, max_results: Option<u32>) -> Result<Vec<FileSearchResult>, String> {
    let max = max_results.unwrap_or(50);
    
    // Build smart query with wildcards
    let smart_query = build_smart_query(&query);
    
    if smart_query.is_empty() {
        return Ok(Vec::new());
    }
    
    tracing::debug!("Everything query: {} -> {}", query, smart_query);
    
    // Run search in blocking thread (Everything API is synchronous)
    // Note: We don't use timeout here because Everything is generally fast
    // and timeout can cause issues with the DLL state
    let result = tokio::task::spawn_blocking(move || {
        match EVERYTHING.get() {
            Some(Ok(lib)) => lib.search(&smart_query, max),
            Some(Err(e)) => Err(e.clone()),
            None => Err("Everything not initialized".to_string()),
        }
    })
    .await
    .map_err(|e| format!("Task failed: {}", e))?;
    
    // Filter out undesirable results
    let filtered: Vec<FileSearchResult> = result?
        .into_iter()
        .filter(|r| {
            let name_lower = r.filename.to_lowercase();
            // Skip uninstallers
            !name_lower.contains("uninstall") 
                && !name_lower.contains("卸载")
                // Skip system/temp files
                && !r.path.contains("$Recycle.Bin")
                && !r.path.contains("System Volume Information")
        })
        .collect();
    
    Ok(filtered)
}

/// Check if Everything is available
pub fn is_available() -> bool {
    matches!(EVERYTHING.get(), Some(Ok(_)))
}

// ═══════════════════════════════════════════════════════════════════════════════
// Tests
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_smart_query_single() {
        assert_eq!(build_smart_query("chrome"), "*chrome*");
        assert_eq!(build_smart_query("IDEA"), "*IDEA*");
    }

    #[test]
    fn test_build_smart_query_with_extension() {
        // Filenames with extensions should be kept together
        assert_eq!(build_smart_query("pkg_bsaml.pck"), "*pkg_bsaml.pck*");
        assert_eq!(build_smart_query("document.pdf"), "*document.pdf*");
        assert_eq!(build_smart_query("test.txt"), "*test.txt*");
    }

    #[test]
    fn test_build_smart_query_multiple() {
        assert_eq!(build_smart_query("google chrome"), "*google* *chrome*");
        assert_eq!(build_smart_query("intel idea"), "*intel* *idea*");
    }

    #[test]
    fn test_build_smart_query_empty() {
        assert_eq!(build_smart_query(""), "");
        assert_eq!(build_smart_query("   "), "");
    }
    
    #[test]
    fn test_classify_file() {
        // .exe is always Application
        assert_eq!(classify_file("C:\\Program Files\\app.exe", "exe"), "Application");
        
        // .lnk in Recent is File
        assert_eq!(classify_file("C:\\Users\\test\\AppData\\Roaming\\Microsoft\\Windows\\Recent\\doc.lnk", "lnk"), "File");
        
        // .lnk in Start Menu is Application
        assert_eq!(classify_file("C:\\ProgramData\\Microsoft\\Windows\\Start Menu\\Programs\\Chrome.lnk", "lnk"), "Application");
        
        // .lnk on Desktop is Application
        assert_eq!(classify_file("C:\\Users\\test\\Desktop\\App.lnk", "lnk"), "Application");
        
        // Regular files are File
        assert_eq!(classify_file("C:\\docs\\file.pdf", "pdf"), "File");
        assert_eq!(classify_file("C:\\data\\file.pck", "pck"), "File");
    }
}
