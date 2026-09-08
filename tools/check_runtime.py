from __future__ import annotations

import importlib.metadata
import os
import platform
import sys

EXPECTED = "0.2.3"


def main() -> int:
    print("=== AstrBot meme-generator runtime check ===")
    print("Python:", sys.executable)
    print("Python version:", platform.python_version())
    print("MEME_HOME:", os.environ.get("MEME_HOME", "<未设置>"))
    try:
        version = importlib.metadata.version("meme-generator")
    except importlib.metadata.PackageNotFoundError:
        print("meme-generator: 未安装在这个 Python 环境")
        return 2
    print("meme-generator:", version)
    if version != EXPECTED:
        print(f"WARNING: 本 DLL 项目严格面向 {EXPECTED}，当前是 {version}。不要加载构建出的 DLL。")
        return 1
    print("OK: 运行时版本与项目目标一致。")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
