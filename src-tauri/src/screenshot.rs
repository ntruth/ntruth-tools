use anyhow::Result;
use base64::{engine::general_purpose::STANDARD, Engine};
use chrono::{DateTime, Utc};
use image::{ImageBuffer, Rgba, ImageEncoder};
use once_cell::sync::Lazy;
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScreenshotItem {
  pub id: String,
  pub data_url: String,
  pub created_at: DateTime<Utc>,
  pub pinned: bool,
  pub note: Option<String>,
}

static COLLECTION: Lazy<Mutex<Vec<ScreenshotItem>>> = Lazy::new(|| Mutex::new(Vec::new()));

fn generate_id() -> String {
  format!("shot-{}", Utc::now().timestamp_millis())
}

pub fn list() -> Vec<ScreenshotItem> {
  let mut items = COLLECTION.lock().clone();
  items.sort_by_key(|item| std::cmp::Reverse(item.created_at));
  items
}

pub fn toggle_pin(id: &str, pinned: bool) -> Result<()> {
  let mut items = COLLECTION.lock();
  if let Some(entry) = items.iter_mut().find(|item| item.id == id) {
    entry.pinned = pinned;
    return Ok(());
  }
  anyhow::bail!("未找到截图记录");
}

pub fn remove(id: &str) -> Result<()> {
  let mut items = COLLECTION.lock();
  let original = items.len();
  items.retain(|item| item.id != id);
  if items.len() == original {
    anyhow::bail!("未找到截图记录");
  }
  Ok(())
}

pub fn capture_stub(note: Option<String>) -> Result<ScreenshotItem> {
  let width: u32 = 320;
  let height: u32 = 200;
  let mut buffer = ImageBuffer::<Rgba<u8>, _>::new(width, height);

  for (x, y, pixel) in buffer.enumerate_pixels_mut() {
    let gradient = ((x + y) % 255) as u8;
    *pixel = Rgba([gradient, 140, 220, 255]);
  }

  let mut bytes: Vec<u8> = Vec::new();
  image::codecs::png::PngEncoder::new(&mut bytes).write_image(
    buffer.as_raw(),
    width,
    height,
    image::ColorType::Rgba8,
  )?;

  let encoded = STANDARD.encode(bytes);
  let data_url = format!("data:image/png;base64,{}", encoded);

  let item = ScreenshotItem {
    id: generate_id(),
    data_url,
    created_at: Utc::now(),
    pinned: false,
    note,
  };

  let mut items = COLLECTION.lock();
  items.insert(0, item.clone());

  Ok(item)
}
