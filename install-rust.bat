@echo off
REM Install Rust on Windows
REM This script will download and install rustup (Rust installer)

setlocal enabledelayedexpansion

echo =========================================
echo Rust Installation Helper
echo =========================================
echo.

REM Check if Rust is already installed
cargo --version >nul 2>&1
if %errorlevel% equ 0 (
    echo Rust is already installed:
    cargo --version
    rustc --version
    goto end
)

echo Rust not found. Proceeding with installation...
echo.

REM Create temp directory if it doesn't exist
if not exist "%USERPROFILE%\AppData\Local\Temp" mkdir "%USERPROFILE%\AppData\Local\Temp"

REM Download rustup-init.exe
echo Downloading Rust installer...
echo Please wait...
echo.

REM Using PowerShell to download (more reliable)
powershell -Command "[Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12, [Net.SecurityProtocolType]::Tls11; (New-Object System.Net.WebClient).DownloadFile('https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe', '%USERPROFILE%\AppData\Local\Temp\rustup-init.exe')"

if not exist "%USERPROFILE%\AppData\Local\Temp\rustup-init.exe" (
    echo.
    echo ERROR: Failed to download Rust installer
    echo Please download it manually from: https://rustup.rs/
    echo.
    pause
    goto end
)

echo Running installer...
echo.
"%USERPROFILE%\AppData\Local\Temp\rustup-init.exe"

if %errorlevel% equ 0 (
    echo.
    echo =========================================
    echo Rust installed successfully!
    echo =========================================
    echo.
    echo Close and reopen your terminal, then run:
    echo   cargo --version
    echo   rustc --version
    echo.
) else (
    echo.
    echo ERROR: Installation failed
    echo.
)

:end
pause
