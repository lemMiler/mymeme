# memelite_custom_memes_rs

将旧版 `meme-generator < 0.1.14` 的 36 个 Python meme 手工迁移为 `meme-generator-rs / meme-generator 0.2.x` 可加载的 Rust 外部 Meme Pack。

项目目录结构参照官方 `MemeCrafters/meme-generator-contrib-rs`：

```text
memelite_custom_memes_rs/
├─ .github/workflows/build.yml
├─ Cargo.toml
├─ src/
├─ resources/
│  └─ images/
│     ├─ behind_do/
│     ├─ fleshlight_air_play/
│     ├─ ...
│     └─ spraypee/
├─ tools/
│  ├─ import_images.py
│  ├─ validate_resources.py
│  └─ resource_manifest.json
├─ import_images.bat
├─ validate_resources.bat
└─ config.example.toml
```

## 1. 图片应该放在哪里

旧 Python 项目结构：

```text
旧项目/
├─ behind_do/
│  ├─ __init__.py
│  └─ images/
│     ├─ 0.png
│     ├─ 1.png
│     └─ ...
├─ fleshlight_air_play/
│  ├─ __init__.py
│  └─ images/
│     └─ 0.png
└─ ...
```

Rust 项目需要改成官方 contrib-rs 风格：

```text
resources/images/
├─ behind_do/
│  ├─ 0.png
│  ├─ 1.png
│  └─ ...
├─ fleshlight_air_play/
│  └─ 0.png
└─ ...
```

也就是：

```text
旧项目/<meme>/images/<文件>
        ↓
新项目/resources/images/<meme>/<文件>
```

不要再保留中间那层 `images`。

## 2. 一键导入旧项目图片（推荐）

Windows 双击：

```text
import_images.bat
```

输入旧版 meme 总目录，例如：

```text
D:\meme-generator-contrib\memes
```

也可以直接输入一个包含这些 meme 目录的 ZIP。

脚本会自动找到所有 `<meme>/images`，把其中图片复制到当前仓库的 `resources/images/<meme>`。

命令行方式：

```powershell
python tools/import_images.py "D:\旧版meme目录" --clean
```

或者：

```powershell
python tools/import_images.py "D:\旧版meme.zip" --clean
```

`--clean` 会先清空对应 meme 的目标资源目录，防止旧文件残留。

## 3. 导入后检查资源

```powershell
python tools/validate_resources.py
```

或者双击：

```text
validate_resources.bat
```

只有显示：

```text
✅ 必需资源完整，可以提交 GitHub 构建。
```

再提交仓库。

## 4. 上传 GitHub 并自动编译

新建 GitHub 仓库，把本项目根目录全部上传并推送到 `main`。

GitHub Actions 会自动：

1. 检查图片资源是否完整；
2. 安装 Rust；
3. `cargo check`；
4. 编译 `x86_64-pc-windows-msvc` DLL；
5. 将 DLL 与 `resources/images` 一起整理成可安装 ZIP；
6. 上传为 Actions Artifact。

也可以在 GitHub：

```text
Actions → Build Windows DLL + resources → Run workflow
```

手动触发。

## 5. 下载构建结果

Actions 成功后下载：

```text
memelite-custom-memes-windows-x64
```

其中包含：

```text
memelite_custom_memes-windows-x64.zip
```

解压后的内容：

```text
libraries/
└─ memelite_custom_memes.dll

resources/
└─ images/
   ├─ behind_do/
   ├─ ...
   └─ spraypee/

config.example.toml
```

## 6. 安装到 meme-generator-rs

先关闭 AstrBot。

把构建 ZIP 解压后的：

```text
libraries/
resources/
```

合并到你的 `%MEME_HOME%`：

```text
%MEME_HOME%/
├─ libraries/
│  └─ memelite_custom_memes.dll
└─ resources/
   └─ images/
      └─ ...
```

确认 `%MEME_HOME%/config.toml` 中启用外部 meme：

```toml
[meme]
load_builtin_memes = true
load_external_memes = true
```

然后重新启动 AstrBot。

> `config.example.toml` 只是示例。不要直接覆盖你已有的完整 `config.toml`，只确认其中 `load_external_memes = true` 即可。

## 7. 当前兼容目标

Cargo 依赖按当前官方 contrib-rs 基线设置：

```toml
skia-safe = { version = "0.93", features = ["textlayout"] }
meme_generator_core = "0.0.5"
meme_generator_utils = "0.0.9"
```

目标为 `meme-generator 0.2.x` Rust binding / meme-generator-rs 外部 Meme Pack。

## 注意

本交付包没有包含你删除掉的原始图片素材，因此仓库中的 `resources/images/<meme>/` 目前只有 `.gitkeep` 占位文件。请在上传 GitHub 前运行 `import_images.bat` 导入你本地原始图片。
