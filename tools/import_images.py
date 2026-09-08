from __future__ import annotations

import argparse
import shutil
import sys
import tempfile
import zipfile
from pathlib import Path

IMAGE_EXTENSIONS = {
    ".jpg", ".jpeg", ".png", ".gif", ".bmp",
    ".webp", ".tiff", ".ico", ".svg",
}


def repo_root() -> Path:
    return Path(__file__).resolve().parents[1]


def extract_if_zip(source: Path):
    if source.is_file() and source.suffix.lower() == ".zip":
        tmp = tempfile.TemporaryDirectory(prefix="memelite_images_")
        with zipfile.ZipFile(source, "r") as zf:
            zf.extractall(tmp.name)
        return Path(tmp.name), tmp
    return source, None


def find_images_dirs(source: Path) -> list[Path]:
    return sorted(
        p for p in source.rglob("*")
        if p.is_dir() and p.name.lower() == "images"
    )


def import_images(source: Path, target_root: Path, clean: bool, move: bool) -> int:
    source, temp = extract_if_zip(source)
    try:
        images_dirs = find_images_dirs(source)
        if not images_dirs:
            print("❌ 没有找到任何名为 images 的文件夹。")
            return 2

        copied = 0
        skipped_non_image = 0
        meme_count = 0

        for images_dir in images_dirs:
            meme_name = images_dir.parent.name
            if not meme_name:
                continue

            files = [
                p for p in images_dir.rglob("*")
                if p.is_file() and p.suffix.lower() in IMAGE_EXTENSIONS
            ]
            if not files:
                print(f"⚠ {meme_name}: images 目录没有图片，跳过")
                continue

            dest = target_root / meme_name
            if clean and dest.exists():
                shutil.rmtree(dest)
            dest.mkdir(parents=True, exist_ok=True)

            for src_file in files:
                rel = src_file.relative_to(images_dir)
                dst_file = dest / rel
                dst_file.parent.mkdir(parents=True, exist_ok=True)
                if move and temp is None:
                    shutil.move(str(src_file), str(dst_file))
                else:
                    shutil.copy2(src_file, dst_file)
                copied += 1

            for p in images_dir.rglob("*"):
                if p.is_file() and p.suffix.lower() not in IMAGE_EXTENSIONS:
                    skipped_non_image += 1

            gitkeep = dest / ".gitkeep"
            if gitkeep.exists():
                gitkeep.unlink()

            meme_count += 1
            print(f"✅ {meme_name}: {len(files)} 张 → {dest}")

        print("\n" + "=" * 62)
        print(f"完成：导入 {meme_count} 个 meme，共 {copied} 张图片")
        if skipped_non_image:
            print(f"忽略非图片文件：{skipped_non_image} 个")
        print(f"目标：{target_root}")
        print("接下来运行：python tools/validate_resources.py")
        print("=" * 62)
        return 0
    finally:
        if temp is not None:
            temp.cleanup()


def main() -> int:
    parser = argparse.ArgumentParser(
        description="把旧版 <meme>/images/* 自动导入 Rust 仓库 resources/images/<meme>/*"
    )
    parser.add_argument("source", nargs="?", help="旧版 meme 根目录或 ZIP")
    parser.add_argument(
        "--target",
        default=str(repo_root() / "resources" / "images"),
        help="目标 resources/images 目录（默认自动定位当前仓库）",
    )
    parser.add_argument(
        "--clean",
        action="store_true",
        help="导入每个 meme 前清空对应目标目录，避免残留旧资源",
    )
    parser.add_argument(
        "--move",
        action="store_true",
        help="移动而不是复制（ZIP 输入时仍使用复制）",
    )
    args = parser.parse_args()

    source_text = args.source
    if not source_text:
        source_text = input("请输入旧版 meme 根目录或 ZIP 路径：").strip().strip('"')
    if not source_text:
        print("❌ 未输入路径")
        return 2

    source = Path(source_text).expanduser().resolve()
    if not source.exists():
        print(f"❌ 源路径不存在：{source}")
        return 2

    target = Path(args.target).expanduser().resolve()
    target.mkdir(parents=True, exist_ok=True)
    return import_images(source, target, args.clean, args.move)


if __name__ == "__main__":
    raise SystemExit(main())
