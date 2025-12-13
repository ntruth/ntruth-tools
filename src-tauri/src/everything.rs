//! Everything64.dll Integration for Windows
//! 
//! This module provides FFI bindings to voidtools' Everything search engine.
//! Everything must be running in the background for search to work.

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
type EverythingQueryW = unsafe extern "system" fn(c_int) -> c_int;
type EverythingGetNumResults = unsafe extern "system" fn() -> c_uint;
type EverythingGetResultFullPathNameW = unsafe extern "system" fn(c_uint, *mut u16, c_uint) -> c_uint;
type EverythingGetLastError = unsafe extern "system" fn() -> c_uint;
type EverythingReset = unsafe extern "system" fn();
type EverythingCleanUp = unsafe extern "system" fn();
type EverythingGetResultSize = unsafe extern "system" fn(c_uint, *mut i64) -> c_int;
type EverythingGetResultDateModified = unsafe extern "system" fn(c_uint, *mut i64) -> c_int;

// Request flags
const EVERYTHING_REQUEST_FILE_NAME: c_uint = 0x00000001;
const EVERYTHING_REQUEST_PATH: c_uint = 0x00000002;
const EVERYTHING_REQUEST_FULL_PATH_AND_FILE_NAME: c_uint = 0x00000004;
const EVERYTHING_REQUEST_SIZE: c_uint = 0x00000010;
const EVERYTHING_REQUEST_DATE_MODIFIED: c_uint = 0x00000040;

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
            
            // Get all symbols first, then dereference them
            let set_search_w = *lib
                .get::<EverythingSetSearchW>(b"Everything_SetSearchW")
                .map_err(|e| format!("Failed to get Everything_SetSearchW: {}", e))?;
            
            let set_request_flags = *lib
                .get::<EverythingSetRequestFlags>(b"Everything_SetRequestFlags")
                .map_err(|e| format!("Failed to get Everything_SetRequestFlags: {}", e))?;
            
            let set_max = *lib
                .get::<EverythingSetMax>(b"Everything_SetMax")
                .map_err(|e| format!("Failed to get Everything_SetMax: {}", e))?;
            
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
    
    fn search(&self, query: &str, max_results: u32) -> Result<Vec<EverythingResult>, String> {
        unsafe {
            // Reset state
            (self.reset)();
            
            // Convert query to wide string
            let query_wide: Vec<u16> = query.encode_utf16().chain(std::iter::once(0)).collect();
            
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
                    
                    // Determine if it's a folder
                    let is_folder = std::path::Path::new(&path)
                        .extension()
                        .is_none() && size == 0;
                    
                    results.push(EverythingResult {
                        path,
                        size: if size > 0 { Some(size as u64) } else { None },
                        date_modified: if date_modified > 0 { Some(date_modified) } else { None },
                        is_folder,
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
pub struct EverythingResult {
    pub path: String,
    pub size: Option<u64>,
    pub date_modified: Option<i64>,
    pub is_folder: bool,
}

/// Initialize Everything DLL
/// 
/// Resolves the DLL path correctly for both dev and production:
/// - Dev: src-tauri/libs/Everything64.dll
/// - Production: resources/libs/Everything64.dll (bundled)
pub fn init_everything(app_handle: &tauri::AppHandle) -> Result<(), String> {
    let dll_path = match resolve_dll_path(app_handle) {
        Ok(path) => path,
        Err(e) => {
            println!("DLL Path Resolution Error: {:?}", e);
            tracing::error!("DLL Path Resolution Error: {:?}", e);
            return Err(e);
        }
    };
    
    println!("Loading Everything64.dll from: {:?}", dll_path);
    tracing::info!("Loading Everything64.dll from: {:?}", dll_path);
    
    let result = EverythingLib::new(&dll_path);
    
    if let Err(ref e) = result {
        println!("DLL Load Error: {:?}", e);
        tracing::error!("DLL Load Error: {:?}", e);
    }
    
    EVERYTHING.get_or_init(|| result);
    
    match EVERYTHING.get() {
        Some(Ok(_)) => {
            println!("Everything64.dll loaded successfully!");
            tracing::info!("Everything64.dll loaded successfully!");
            Ok(())
        },
        Some(Err(e)) => {
            println!("Everything initialization failed: {:?}", e);
            Err(e.clone())
        },
        None => Err("Failed to initialize Everything".to_string()),
    }
}

/// Resolve DLL path for both dev and production environments
fn resolve_dll_path(app_handle: &tauri::AppHandle) -> Result<PathBuf, String> {
    use tauri::path::BaseDirectory;
    
    println!("=== DLL Path Resolution Debug ===");
    
    // Method 1: Use Tauri 2.0's resolve API with BaseDirectory::Resource (RECOMMENDED)
    if let Ok(resource_path) = app_handle.path().resolve("libs/Everything64.dll", BaseDirectory::Resource) {
        println!("Resolved resource path: {:?}", resource_path);
        if resource_path.exists() {
            println!("✓ Found DLL via BaseDirectory::Resource");
            return Ok(resource_path);
        } else {
            println!("✗ Resource path does not exist: {:?}", resource_path);
        }
    } else {
        println!("✗ Failed to resolve via BaseDirectory::Resource");
    }
    
    // Method 2: Try resource_dir() for production builds
    if let Ok(resource_dir) = app_handle.path().resource_dir() {
        let production_path = resource_dir.join("libs").join("Everything64.dll");
        println!("Trying production path: {:?}", production_path);
        if production_path.exists() {
            println!("✓ Found DLL in production resource dir");
            return Ok(production_path);
        }
    }
    
    // Method 3: Development fallback paths
    let exe_path = std::env::current_exe()
        .map_err(|e| format!("Failed to get exe path: {}", e))?;
    let exe_dir = exe_path
        .parent()
        .ok_or("Failed to get exe directory")?
        .to_path_buf();
    
    println!("Exe directory: {:?}", exe_dir);
    
    // In dev mode, we're in target/debug, so try multiple relative paths
    let dev_paths = [
        // Direct libs folder next to exe
        exe_dir.join("libs").join("Everything64.dll"),
        // Go up from target/debug to src-tauri/libs
        exe_dir.join("..").join("..").join("libs").join("Everything64.dll"),
        // Go up further if in nested target directory
        exe_dir.join("..").join("..").join("..").join("src-tauri").join("libs").join("Everything64.dll"),
        // Try relative to cwd
        PathBuf::from("libs").join("Everything64.dll"),
        // Try src-tauri relative to cwd
        PathBuf::from("src-tauri").join("libs").join("Everything64.dll"),
    ];
    
    for (i, path) in dev_paths.iter().enumerate() {
        println!("Trying dev path {}: {:?}", i + 1, path);
        
        // Try canonical path first
        if let Ok(canonical) = path.canonicalize() {
            println!("  Canonical: {:?}", canonical);
            if canonical.exists() {
                println!("✓ Found DLL via dev path {} (canonical)", i + 1);
                return Ok(canonical);
            }
        }
        
        // Try direct path
        if path.exists() {
            println!("✓ Found DLL via dev path {}", i + 1);
            return Ok(path.clone());
        }
    }
    
    // Method 4: Try current working directory
    if let Ok(cwd) = std::env::current_dir() {
        println!("Current working directory: {:?}", cwd);
        let cwd_path = cwd.join("libs").join("Everything64.dll");
        let cwd_src_tauri_path = cwd.join("src-tauri").join("libs").join("Everything64.dll");
        
        if cwd_path.exists() {
            println!("✓ Found DLL in cwd/libs");
            return Ok(cwd_path);
        }
        if cwd_src_tauri_path.exists() {
            println!("✓ Found DLL in cwd/src-tauri/libs");
            return Ok(cwd_src_tauri_path);
        }
    }
    
    println!("=== DLL Path Resolution Failed ===");
    Err(format!(
        "Everything64.dll not found. Checked paths:\n\
        - BaseDirectory::Resource\n\
        - resource_dir()/libs\n\
        - Multiple dev paths relative to exe: {:?}\n\
        - Current working directory\n\
        Please ensure Everything64.dll is in src-tauri/libs/",
        exe_dir
    ))
}

/// Search using Everything
/// 
/// This is the Tauri command exposed to the frontend.
#[tauri::command]
pub async fn everything_search(
    query: String,
    max_results: Option<u32>,
) -> Result<Vec<EverythingResult>, String> {
    let max = max_results.unwrap_or(50);
    
    match EVERYTHING.get() {
        Some(Ok(lib)) => lib.search(&query, max),
        Some(Err(e)) => Err(format!("Everything not initialized: {}", e)),
        None => Err("Everything not initialized".to_string()),
    }
}

/// Check if Everything is available
pub fn is_everything_available() -> bool {
    matches!(EVERYTHING.get(), Some(Ok(_)))
}
