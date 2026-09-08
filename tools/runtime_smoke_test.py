"""Runtime ABI smoke test for the built external meme DLL.

This intentionally invokes the real meme-generator Python binding.  A Rust ABI
mismatch often terminates the Python process instead of raising a Python
exception, so GitHub Actions must run this *before* publishing an artifact.
"""
from __future__ import annotations

import importlib.metadata
import os
import struct
import sys
import zlib
from pathlib import Path

EXPECTED_VERSION = "0.2.3"


def make_png(width: int = 512, height: int = 512) -> bytes:
    """Create an opaque RGBA PNG using only the Python standard library."""
    # Keep the image deliberately simple; the test is for FFI + image pipeline,
    # not visual correctness.
    pixel = bytes((120, 170, 220, 255))
    raw = b"".join(b"\x00" + pixel * width for _ in range(height))

    def chunk(kind: bytes, data: bytes) -> bytes:
        payload = kind + data
        return (
            struct.pack(">I", len(data))
            + payload
            + struct.pack(">I", zlib.crc32(payload) & 0xFFFFFFFF)
        )

    return (
        b"\x89PNG\r\n\x1a\n"
        + chunk(b"IHDR", struct.pack(">IIBBBBB", width, height, 8, 6, 0, 0, 0))
        + chunk(b"IDAT", zlib.compress(raw, 6))
        + chunk(b"IEND", b"")
    )


def require_bytes(name: str, value: object) -> bytes:
    if not isinstance(value, bytes):
        raise RuntimeError(f"{name}: expected bytes, got {type(value)!r}: {value!r}")
    if len(value) < 128:
        raise RuntimeError(f"{name}: suspiciously small output: {len(value)} bytes")
    return value


def main() -> int:
    version = importlib.metadata.version("meme-generator")
    print(f"meme-generator Python package: {version}")
    print(f"MEME_HOME: {os.environ.get('MEME_HOME', '<unset>')}")
    if version != EXPECTED_VERSION:
        raise RuntimeError(
            f"Smoke test requires meme-generator=={EXPECTED_VERSION}, got {version}"
        )

    # Import only after MEME_HOME has been configured by the workflow.  External
    # libraries are discovered during initialization/loading.
    from meme_generator import Image, get_meme

    avatar = Image("abi-smoke", make_png())

    # 1-image animated template: exercises external registration, Skia image
    # operations, load_image and our direct GifEncoder path.
    animated = get_meme("sitdown_do")
    if animated is None:
        raise RuntimeError("External meme 'sitdown_do' was not registered")
    out_gif = require_bytes(
        "sitdown_do",
        animated.generate([avatar], [], {}),
    )
    if out_gif[:3] != b"GIF":
        raise RuntimeError(f"sitdown_do: expected GIF output, header={out_gif[:8]!r}")
    print(f"OK sitdown_do: {len(out_gif)} bytes")

    # 2-image static template: exercises Vec<InputImage> crossing the external
    # Rust ABI and make_png_or_gif on a second code path.
    avatar_static_1 = Image("abi-smoke-static-1", make_png())
    avatar_static_2 = Image("abi-smoke-static-2", make_png(384, 384))
    static = get_meme("mihoyo_elysia_come")
    if static is None:
        raise RuntimeError("External meme 'mihoyo_elysia_come' was not registered")
    out_png = require_bytes(
        "mihoyo_elysia_come",
        static.generate([avatar_static_1, avatar_static_2], [], {}),
    )
    if out_png[:8] != b"\x89PNG\r\n\x1a\n":
        # make_png_or_gif can theoretically preserve animated input, but both
        # inputs here are static PNGs, so PNG is required.
        raise RuntimeError(
            f"mihoyo_elysia_come: expected PNG output, header={out_png[:8]!r}"
        )
    print(f"OK mihoyo_elysia_come: {len(out_png)} bytes")

    print("Runtime ABI smoke test passed.")
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except Exception as exc:
        print(f"SMOKE TEST FAILED: {exc}", file=sys.stderr)
        raise
