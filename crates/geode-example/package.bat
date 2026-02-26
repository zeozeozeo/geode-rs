@echo off
setlocal enabledelayedexpansion

set MOD_ID=test.example_mod
set EXAMPLE_DIR=%~dp0

pushd %EXAMPLE_DIR%..\..
set PROJECT_ROOT=%CD%
popd

cd /d %PROJECT_ROOT%

set BINARY_ARGS=
set BUILD_FAILED=0

echo Building %MOD_ID% for Windows...
cargo build --release -p geode-example
if %errorlevel% neq 0 (
    echo Windows build failed.
    set BUILD_FAILED=1
) else (
    set WIN_DLL=%PROJECT_ROOT%\target\release\geode_example.dll
    if not exist "!WIN_DLL!" (
        echo Could not find Windows DLL: !WIN_DLL!
        set BUILD_FAILED=1
    ) else (
        set WIN_DLL_NAMED=%PROJECT_ROOT%\target\release\%MOD_ID%.dll
        copy /Y "!WIN_DLL!" "!WIN_DLL_NAMED!" >nul
        set BINARY_ARGS=!BINARY_ARGS! --binary "!WIN_DLL_NAMED!"
        echo   Built: !WIN_DLL_NAMED!
    )
)

where cargo-ndk >nul 2>&1
if %errorlevel% neq 0 (
    echo Skipping Android builds: cargo-ndk not found. Install with: cargo install cargo-ndk
    goto :package
)

echo.
echo Building %MOD_ID% for Android arm64-v8a...
cargo ndk --target aarch64-linux-android --platform 21 -- build --release -p geode-example
if %errorlevel% neq 0 (
    echo Android64 build failed.
    set BUILD_FAILED=1
) else (
    set SO64=%PROJECT_ROOT%\target\aarch64-linux-android\release\libgeode_example.so
    if not exist "!SO64!" (
        echo Could not find Android64 .so: !SO64!
        set BUILD_FAILED=1
    ) else (

        set SO64_NAMED=%PROJECT_ROOT%\target\aarch64-linux-android\release\%MOD_ID%.android64.so
        copy /Y "!SO64!" "!SO64_NAMED!" >nul
        set BINARY_ARGS=!BINARY_ARGS! --binary "!SO64_NAMED!"
        echo   Built: !SO64_NAMED!
    )
)

echo.
echo Building %MOD_ID% for Android armeabi-v7a...
cargo ndk --target armv7-linux-androideabi --platform 21 -- build --release -p geode-example
if %errorlevel% neq 0 (
    echo Android32 build failed.
    set BUILD_FAILED=1
) else (
    set SO32=%PROJECT_ROOT%\target\armv7-linux-androideabi\release\libgeode_example.so
    if not exist "!SO32!" (
        echo Could not find Android32 .so: !SO32!
        set BUILD_FAILED=1
    ) else (
        set SO32_NAMED=%PROJECT_ROOT%\target\armv7-linux-androideabi\release\%MOD_ID%.android32.so
        copy /Y "!SO32!" "!SO32_NAMED!" >nul
        set BINARY_ARGS=!BINARY_ARGS! --binary "!SO32_NAMED!"
        echo   Built: !SO32_NAMED!
    )
)

:package

if "%BINARY_ARGS%"=="" (
    echo No binaries were built successfully. Aborting.
    exit /b 1
)

if "%BUILD_FAILED%"=="1" (
    echo.
    echo Warning: one or more targets failed to build. Packaging what succeeded.
)

echo.
echo Packaging %MOD_ID%.geode...
set OUTPUT=%EXAMPLE_DIR%%MOD_ID%.geode
geode package new "%EXAMPLE_DIR%." %BINARY_ARGS% --output "%OUTPUT%"
if %errorlevel% neq 0 (
    echo Packaging failed.
    exit /b %errorlevel%
)

echo Created %OUTPUT%
echo.
echo To install on Android, copy to:
echo   /storage/emulated/0/Android/media/com.geode.launcher/game/geode/mods/
