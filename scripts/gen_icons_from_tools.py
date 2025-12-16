from __future__ import annotations

from pathlib import Path

from PIL import Image


def _contain_rgba(img: Image.Image, size: int) -> Image.Image:
    """Resize image into a size×size canvas while preserving aspect ratio."""
    img = img.convert("RGBA")
    src_w, src_h = img.size
    if src_w == 0 or src_h == 0:
        raise ValueError("Invalid source image size")

    scale = min(size / src_w, size / src_h)
    dst_w = max(1, int(round(src_w * scale)))
    dst_h = max(1, int(round(src_h * scale)))

    resized = img.resize((dst_w, dst_h), resample=Image.Resampling.LANCZOS)
    canvas = Image.new("RGBA", (size, size), (0, 0, 0, 0))
    ox = (size - dst_w) // 2
    oy = (size - dst_h) // 2
    canvas.paste(resized, (ox, oy), resized)
    return canvas


def main() -> None:
    repo = Path(r"e:\Code\ntruth-tools")
    icons_dir = repo / "src-tauri" / "icons"

    src = icons_dir / "tools.png"
    if not src.exists():
        raise SystemExit(f"Missing source icon: {src}")

    # Outputs requested
    out_ico = icons_dir / "icon.ico"
    out_png = icons_dir / "icon.png"

    base = Image.open(src)

    # Generate a crisp PNG for general use. 512 is a common app icon size.
    png512 = _contain_rgba(base, 512)
    png512.save(out_png, format="PNG", optimize=True)

    # Generate a multi-size ICO. Windows will pick the closest.
    ico_sizes = [16, 24, 32, 48, 64, 128, 256]
    ico_imgs = [_contain_rgba(base, s) for s in ico_sizes]

    # Pillow writes multiple frames when sizes are provided.
    ico_imgs[0].save(
        out_ico,
        format="ICO",
        sizes=[(s, s) for s in ico_sizes],
    )

    print("Wrote", out_png)
    print("Wrote", out_ico)


if __name__ == "__main__":
    main()
