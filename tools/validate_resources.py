from __future__ import annotations

import json
import sys
from pathlib import Path

# Windows 控制台默认可能是 GBK，无法输出 emoji，统一按 UTF-8 处理
for _stream in (sys.stdout, sys.stderr):
    try:
        _stream.reconfigure(encoding="utf-8")
    except Exception:
        pass

ROOT = Path(__file__).resolve().parents[1]
RESOURCE_ROOT = ROOT / "resources" / "images"
MANIFEST = ROOT / "tools" / "resource_manifest.json"


def main() -> int:
    manifest = json.loads(MANIFEST.read_text(encoding="utf-8"))
    missing: list[str] = []
    present = 0

    for meme_name, files in manifest.items():
        for filename in files:
            path = RESOURCE_ROOT / meme_name / filename
            if path.is_file():
                present += 1
            else:
                missing.append(str(path.relative_to(ROOT)))

    expected = present + len(missing)
    print(f"资源检查：{present}/{expected} 个必需文件已存在")

    if missing:
        print("\n❌ 缺少以下资源：")
        for item in missing:
            print(f"  - {item}")
        print("\n请先运行 tools/import_images.py 导入旧项目 images。")
        return 1

    print("✅ 必需资源完整，可以提交 GitHub 构建。")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
