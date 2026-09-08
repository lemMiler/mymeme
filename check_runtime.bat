@echo off
setlocal
cd /d "%~dp0"
python tools\check_runtime.py
set code=%errorlevel%
echo.
if not "%code%"=="0" (
  echo [FAIL] 请先处理上面的版本问题，不要安装 DLL。
) else (
  echo [OK] 可以继续使用针对 meme-generator 0.2.3 构建的 DLL。
)
pause
exit /b %code%
