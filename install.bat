@echo off
setlocal

echo =======================================================
echo          TOM - Tool Manager Windows Installer
echo =======================================================
echo.

:: 1. Check for Cargo / Rust toolchain
where cargo >nul 2>nul
if %ERRORLEVEL% neq 0 (
    echo [ERROR] Rust and Cargo were not found in your PATH.
    echo Please install Rust from https://rustup.rs/ and try again.
    echo.
    exit /b 1
)

echo [*] Rust toolchain detected.

:: 2. Determine directories
set "SCRIPT_DIR=%~dp0"
set "TOM_DIR=%SCRIPT_DIR%tom"
set "TARGET_DIR=%USERPROFILE%\.cargo\bin"

if not exist "%TOM_DIR%\Cargo.toml" (
    echo [ERROR] Could not find TOM source code at "%TOM_DIR%".
    echo Please ensure install.bat is located in the root of the tools repository.
    echo.
    exit /b 1
)

:: 3. Build TOM
echo [*] Compiling TOM...
cd /d "%TOM_DIR%"
cargo build --release >nul 2>nul
if exist "%TOM_DIR%\target\release\tom.exe" (
    set "BUILT_BIN=%TOM_DIR%\target\release\tom.exe"
) else (
    cargo build >nul 2>nul
    if exist "%TOM_DIR%\target\debug\tom.exe" (
        set "BUILT_BIN=%TOM_DIR%\target\debug\tom.exe"
    ) else (
        echo [ERROR] Compilation failed.
        cd /d "%SCRIPT_DIR%"
        exit /b 1
    )
)

:: 4. Ensure destination folder exists
if not exist "%TARGET_DIR%" (
    mkdir "%TARGET_DIR%" 2>nul
)

:: 5. Copy executable to cargo bin
echo [*] Installing tom.exe to %TARGET_DIR%\tom.exe...
copy /Y "%BUILT_BIN%" "%TARGET_DIR%\tom.exe" >nul
if %ERRORLEVEL% neq 0 (
    echo [*] Direct copy locked, attempting force copy...
    powershell -NoProfile -Command "Copy-Item -Path '%BUILT_BIN%' -Destination '%TARGET_DIR%\tom.exe' -Force" >nul 2>nul
)

cd /d "%SCRIPT_DIR%"

:: 6. Verify Installation
echo.
echo =======================================================
echo [*] Verifying installation...
"%TARGET_DIR%\tom.exe" --version
if %ERRORLEVEL% equ 0 (
    echo.
    echo [SUCCESS] TOM has been installed successfully!
    echo.
    echo Useful commands to get started:
    echo   tom list          - View all managed tools and status
    echo   tom status        - View Git status of all tools
    echo   tom fetch all     - Fetch all repositories from registry
    echo   tom install [tool]- Build and install a specific tool
    echo   tom --help        - Show all available commands
    echo =======================================================
) else (
    echo.
    echo [WARNING] Verification command exited with an error.
    echo Make sure %TARGET_DIR% is in your system PATH.
)

echo.