# memelite_custom_memes_rs — ABI 安全构建版

这是将旧版 `meme-generator < 0.1.14` 的 36 个 Python meme 手工迁移到 `meme-generator-rs / meme-generator 0.2.3` 的 Windows 外部 Meme Pack。

> **重要：本版专门修复上一版“DLL 可以编译，但调用 meme 时宿主进程直接崩溃”的构建风险。**
>
> Rust 没有稳定 ABI。外部 meme DLL 与宿主 `meme-generator-rs` 使用不同 Rust 编译器版本时，可能不是抛异常，而是直接让 Python/AstrBot 进程崩溃。因此本项目不再使用浮动的 `stable` 工具链，而是固定 `rustc 1.93.1`，并在 GitHub Actions 中用真实的 `meme-generator==0.2.3` 加载 DLL、实际生成 GIF 和 PNG。只有运行时测试通过后才上传 Artifact。

## 兼容目标

本仓库严格面向：

```text
meme-generator Python binding : 0.2.3
meme_generator_core           : 0.0.5
meme_generator_utils          : 0.0.9
skia-safe                     : 0.93.x
Rust                           : 1.93.1
Windows target                 : x86_64-pc-windows-msvc
```

仓库根目录的 `rust-toolchain.toml` 已固定：

```toml
[toolchain]
channel = "1.93.1"
profile = "minimal"
targets = ["x86_64-pc-windows-msvc"]
```

**不要把它改回 `stable`。**

---

## 0. 安装前先检查 AstrBot 实际版本

请在 **AstrBot 真正使用的 Python 环境** 中运行：

```powershell
python -c "import importlib.metadata as m; print(m.version('meme-generator'))"
```

或者双击本仓库：

```text
check_runtime.bat
```

只有显示：

```text
meme-generator: 0.2.3
OK: 运行时版本与项目目标一致。
```

才使用 GitHub 构建出的 DLL。

如果是 `0.2.0 / 0.2.1 / 0.2.2`，**先不要加载本 DLL**。不同宿主版本的 Rust ABI / core 依赖可能不同。

---

## 1. 项目结构

```text
memelite_custom_memes_rs/
├─ .github/workflows/build.yml
├─ Cargo.toml
├─ rust-toolchain.toml
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
│  ├─ runtime_smoke_test.py
│  ├─ check_runtime.py
│  └─ resource_manifest.json
├─ import_images.bat
├─ validate_resources.bat
├─ check_runtime.bat
├─ build_windows.ps1
└─ config.example.toml
```

---

## 2. 把旧 Python 项目里的 images 自动导入

你原来的资源结构：

```text
旧项目/
├─ behind_do/
│  ├─ __init__.py
│  └─ images/
│     ├─ 0.png
│     └─ ...
└─ sitdown_do/
   ├─ __init__.py
   └─ images/
      ├─ 1.png
      ├─ 2.png
      └─ 3.png
```

Rust 外部包结构需要变成：

```text
resources/images/
├─ behind_do/
│  ├─ 0.png
│  └─ ...
└─ sitdown_do/
   ├─ 1.png
   ├─ 2.png
   └─ 3.png
```

即：

```text
旧项目/<meme>/images/<文件>
              ↓
resources/images/<meme>/<文件>
```

### 推荐：双击

```text
import_images.bat
```

然后输入你保存完整图片的旧项目根目录或 ZIP。

命令行也可以：

```powershell
python tools\import_images.py "D:\旧版meme目录" --clean
```

或者：

```powershell
python tools\import_images.py "D:\旧版meme.zip" --clean
```

---

## 3. 验证 36 个 meme 的 168 个必需资源

导入图片后双击：

```text
validate_resources.bat
```

或：

```powershell
python tools\validate_resources.py
```

必须看到：

```text
✅ 必需资源完整，可以提交 GitHub 构建。
```

否则 GitHub Actions 会主动失败，不生成 DLL Artifact。

---

## 4. 上传 GitHub

把**整个项目根目录**上传到你的 GitHub 仓库并推送 `main`：

```powershell
git init
git add .
git commit -m "Rust meme pack"
git branch -M main
git remote add origin <你的仓库地址>
git push -u origin main
```

GitHub：

```text
Actions
→ Build and ABI-test Windows DLL
```

也可以 `Run workflow` 手动运行。

---

## 5. 新版 CI 会真正测试 DLL，而不是只编译

Action 的顺序：

```text
检查 168 个图片资源
        ↓
安装并强制固定 Rust 1.93.1
        ↓
cargo check
        ↓
cargo build --release
        ↓
安装 meme-generator==0.2.3 Windows Python binding
        ↓
建立独立临时 MEME_HOME
        ↓
加载刚编译出的 DLL
        ↓
实际调用 sitdown_do 生成 GIF
        ↓
实际调用 mihoyo_elysia_come 生成双图 PNG
        ↓
只有全部成功才上传 Artifact
```

这里非常关键：如果 DLL 存在 Rust ABI 问题，Python 进程可能直接发生 native crash。GitHub Action 会因此失败，**不会再把这个 DLL 作为“成功构建”发出来。**

测试覆盖两个不同路径：

```text
sitdown_do
→ 1 张输入图
→ 外部资源加载
→ Skia 图像处理
→ 自定义 GifEncoder
→ GIF 输出

mihoyo_elysia_come
→ 2 张输入图
→ Vec<InputImage> 跨外部 Rust ABI
→ make_png_or_gif
→ PNG 输出
```

---

## 6. 下载正确的 Artifact

只有 Action 全绿后，下载：

```text
memelite-custom-memes-windows-x64-abi-tested
```

里面是：

```text
memelite_custom_memes-windows-x64.zip
```

解压结构：

```text
libraries/
└─ memelite_custom_memes.dll

resources/
└─ images/
   ├─ behind_do/
   ├─ ...
   └─ spraypee/

BUILD_INFO.txt
config.example.toml
```

`BUILD_INFO.txt` 会明确写：

```text
meme-generator target: 0.2.3
rustc target: 1.93.1
target: x86_64-pc-windows-msvc
runtime smoke test: PASSED
```

---

## 7. 安装时先删除上一版 DLL

**先完全关闭 AstrBot。**

如果你安装过上一版错误构建的 DLL，先删除：

```text
%MEME_HOME%\libraries\memelite_custom_memes.dll
```

确认 `libraries` 里没有同一个 pack 的旧 DLL、副本 DLL、重命名 DLL，例如：

```text
memelite_custom_memes.dll
memelite_custom_memes_old.dll
memelite_custom_memes (1).dll
```

外部 loader 会扫描 DLL；保留旧副本可能导致旧库也被加载。

然后把新 Artifact ZIP 中的：

```text
libraries/
resources/
```

合并到：

```text
%MEME_HOME%/
```

最终：

```text
%MEME_HOME%/
├─ config.toml
├─ libraries/
│  └─ memelite_custom_memes.dll
└─ resources/
   └─ images/
      └─ ...
```

配置至少包含：

```toml
[meme]
load_builtin_memes = true
load_external_memes = true
```

不要用 `config.example.toml` 覆盖你原来的完整配置。

---

## 8. 如果安装新版本后仍然 native crash

第一步立即移除：

```text
%MEME_HOME%\libraries\memelite_custom_memes.dll
```

再启动 AstrBot。图片文件本身不会注册 Rust 代码，所以保留 `resources/images` 通常不会造成进程级 native crash。

然后保留这三样信息：

1. `check_runtime.bat` 输出；
2. GitHub Action 中 `rustc:` 那一行和 `Runtime ABI smoke test` 结果；
3. AstrBot 崩溃前最后 30~50 行控制台日志，以及具体触发的是哪个 meme 指令。

这样可以继续定位到具体模板代码，而不是再猜 ABI。

---

## 关于本 ZIP 中为什么仍然没有你的原图片

你之前上传给我的代码包已经删除了 `images` 内的真实素材，所以本仓库只提供目录与资源清单。请使用 `import_images.bat` 从你本机保存完整素材的旧项目自动导入。
