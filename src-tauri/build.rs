fn main() {
  tauri_build::build();

  // Ensure portable deployment: copy src-tauri/libs/* next to the built binary
  // (target/{debug|release}/). This makes Everything64.dll available in the exe directory.
  copy_portable_libs();
}

fn copy_portable_libs() {
  use std::env;
  use std::fs;
  use std::path::PathBuf;

  let manifest_dir = match env::var("CARGO_MANIFEST_DIR") {
    Ok(v) => PathBuf::from(v),
    Err(_) => return,
  };

  let libs_dir = manifest_dir.join("libs");
  if !libs_dir.exists() {
    return;
  }

  let profile = env::var("PROFILE").unwrap_or_else(|_| "debug".to_string());
  let target_dir = env::var_os("CARGO_TARGET_DIR")
    .map(PathBuf::from)
    .unwrap_or_else(|| manifest_dir.join("target"));
  let out_dir = target_dir.join(&profile);

  if let Err(e) = fs::create_dir_all(&out_dir) {
    println!("cargo:warning=Failed to create target dir {:?}: {}", out_dir, e);
    return;
  }

  // Copy all files directly under libs/ into target/{profile}/
  // (we only need Everything64.dll, but this keeps behavior consistent if more DLLs are added).
  let entries = match fs::read_dir(&libs_dir) {
    Ok(v) => v,
    Err(e) => {
      println!("cargo:warning=Failed to read libs dir {:?}: {}", libs_dir, e);
      return;
    }
  };

  for entry in entries.flatten() {
    let src = entry.path();
    if !src.is_file() {
      continue;
    }

    if let Some(name) = src.file_name() {
      let dest = out_dir.join(name);
      // Only overwrite if needed (avoid touching timestamps on every build).
      let should_copy = match (fs::metadata(&src), fs::metadata(&dest)) {
        (Ok(sm), Ok(dm)) => sm.len() != dm.len(),
        (Ok(_), Err(_)) => true,
        _ => false,
      };
      if should_copy {
        if let Err(e) = fs::copy(&src, &dest) {
          println!("cargo:warning=Failed to copy {:?} -> {:?}: {}", src, dest, e);
        }
      }

      if let Some(src_str) = src.to_str() {
        println!("cargo:rerun-if-changed={}", src_str);
      }
    }
  }
}
