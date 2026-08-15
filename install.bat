@echo off
setlocal enabledelayedexpansion
cd /d "%~dp0"

echo ======================================================================
echo   TOOLS SUITE - TOM (Tool Manager) Windows Installer
echo ======================================================================
echo.
echo [*] Requirements:
echo     - Rust and Cargo toolchain in PATH (https://rustup.rs/)
echo.
echo [!] Tips:
echo     - Compiles TOM and copies tom.exe to %%USERPROFILE%%\.cargo\bin\
echo     - After installation, 'tom' command is available anywhere in your terminal
echo.

if "%1" neq "-y" if "%1" neq "--yes" (
    set /p "CONFIRM=Proceed with installation of TOM? [Y/n]: "
    if /i "!CONFIRM!"=="n" (
        echo [!] Installation cancelled.
        exit /b 0
    )
)

echo.
echo [1/3] Verifying Cargo environment...
where cargo >nul 2>nul
if %ERRORLEVEL% neq 0 (
    echo [ERROR] 'cargo' is not recognized. Please install Rust from https://rustup.rs
    exit /b 1
)
echo       Rust toolchain detected.

echo.
echo [2/3] Compiling TOM in release mode...
cargo build --manifest-path tom/Cargo.toml --release
if %ERRORLEVEL% neq 0 (
    echo [ERROR] Failed to compile TOM.
    exit /b 1
)

echo.
echo [3/3] Installing tom.exe to %%USERPROFILE%%\.cargo\bin...
if not exist "%USERPROFILE%\.cargo\bin" mkdir "%USERPROFILE%\.cargo\bin" 2>nul
copy /Y "tom\target\release\tom.exe" "%USERPROFILE%\.cargo\bin\tom.exe" >nul
if %ERRORLEVEL% neq 0 (
    powershell -NoProfile -Command "Copy-Item -Path 'tom\target\release\tom.exe' -Destination '%USERPROFILE%\.cargo\bin\tom.exe' -Force" >nul 2>nul
)

echo.
echo ======================================================================
echo   [+] TOM is successfully installed and ready!
echo   Run 'tom status' or 'tom --help' to get started.
echo ======================================================================
echo.