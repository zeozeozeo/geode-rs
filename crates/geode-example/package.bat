@echo off
setlocal enabledelayedexpansion

set MOD_ID=test.example_mod
set EXAMPLE_DIR=%~dp0
set PROJECT_ROOT=%EXAMPLE_DIR%..\..\

cd /d %EXAMPLE_DIR%

echo Building %MOD_ID%...
cargo build --release

set DLL_PATH=%PROJECT_ROOT%target\release\%MOD_ID%.dll
set OUTPUT_PATH=%EXAMPLE_DIR%%MOD_ID%.geode

if not exist "%DLL_PATH%" (
    set ALT_DLL_PATH=%PROJECT_ROOT%target\release\geode_example.dll
    if exist "!ALT_DLL_PATH!" (
        set DLL_PATH=!ALT_DLL_PATH!
    ) else (
        echo Could not find built DLL. Expected: %DLL_PATH%
        exit /b 1
    )
)

echo Packaging %MOD_ID%.geode...
geode package new . --binary "%DLL_PATH%" --output "%OUTPUT_PATH%"

echo Created %OUTPUT_PATH%
