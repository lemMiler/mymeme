@echo off
chcp 65001 >nul
cd /d "%~dp0"
echo ============================================================
echo 旧版 meme images -^> Rust resources/images 自动导入
echo ============================================================

:: 检查 Python 是否可用
python --version >nul 2>&1
if errorlevel 1 (
    echo 错误: 未检测到 Python 环境，请先安装 Python 并添加到环境变量。
    pause
    exit /b 1
)

set /p SOURCE=请输入旧版 meme 根目录或 ZIP 路径: 

:: 运行导入脚本
python tools\import_images.py "%SOURCE%" --clean

if errorlevel 1 (
  echo.
  echo [失败] 导入脚本执行出错。
  pause
  exit /b 1
)

echo.
echo [成功] 正在验证资源...
python tools\validate_resources.py

echo.
echo 全部操作完成。
pause
