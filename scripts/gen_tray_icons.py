from pathlib import Path

from PIL import Image


def main() -> None:
    src = Path(r"e:\Code\ntruth-tools\src-tauri\icons\tools.png")
    out16 = src.with_name("tools-16.png")
    out32 = src.with_name("tools-32.png")

    img = Image.open(src).convert("RGBA")

    img16 = img.resize((16, 16), resample=Image.Resampling.LANCZOS)
    img32 = img.resize((32, 32), resample=Image.Resampling.LANCZOS)

    img16.save(out16, format="PNG", optimize=True)
    img32.save(out32, format="PNG", optimize=True)

    print("Wrote", out16)
    print("Wrote", out32)


if __name__ == "__main__":
    main()
