@echo off
setlocal enabledelayedexpansion
cd /d "%~dp0"

echo ======================================================================
echo   TOM - Tool Manager CLI (Installer)
echo ======================================================================
echo.
echo [*] Requirements:
echo     - Rust and Cargo in PATH (https://rustup.rs/)
echo.
echo [!] Tips:
echo     - Manages git repositories, local toolchains, and installation status
echo     - Installs binary to %USERPROFILE%\.cargo\bin\tom.exe
echo     - Run 'tom --help' to see available commands
echo.

if "%1" neq "-y" if "%1" neq "--yes" (
    set /p "CONFIRM=Proceed with installation? [Y/n]: "
    if /i "!CONFIRM!"=="n" (
        echo [!] Installation cancelled.
        exit /b 0
    )
)

echo.
echo [1/3] Checking Rust toolchain...
where cargo >nul 2>nul
if %ERRORLEVEL% neq 0 (
    echo [ERROR] 'cargo' was not found in PATH. Please install Rust from https://rustup.rs
    exit /b 1
)
echo       Rust toolchain detected.

echo.
echo [2/3] Compiling tom in release mode...
cargo build --release
if %ERRORLEVEL% neq 0 (
    echo [ERROR] Compilation failed.
    exit /b 1
)

echo.
echo [3/3] Installing binary to %USERPROFILE%\.cargo\bin\tom.exe...
if not exist "%USERPROFILE%\.cargo\bin" mkdir "%USERPROFILE%\.cargo\bin" 2>nul
copy /Y "target\release\tom.exe" "%USERPROFILE%\.cargo\bin\tom.exe" >nul
if %ERRORLEVEL% neq 0 (
    powershell -NoProfile -Command "Copy-Item -Path 'target\release\tom.exe' -Destination '%USERPROFILE%\.cargo\bin\tom.exe' -Force" >nul 2>nul
)

echo.
echo ======================================================================
echo   [+] Successfully installed tom!
echo   Try running: tom status
echo ======================================================================
echo.
