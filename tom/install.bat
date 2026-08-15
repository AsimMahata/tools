@echo off
cd /d "%~dp0"
echo [*] Building tom in release mode...
cargo build --release
if %ERRORLEVEL% equ 0 (
    if not exist "%USERPROFILE%\.cargo\bin" mkdir "%USERPROFILE%\.cargo\bin" 2>nul
    copy /Y "target\release\tom.exe" "%USERPROFILE%\.cargo\bin\tom.exe" >nul
    echo [✓] Successfully installed tom to %USERPROFILE%\.cargo\bin\tom.exe
)
