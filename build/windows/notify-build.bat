@ECHO ##### Removing previous builds #####
@rmdir "notify" /s /q

@ECHO ##### Starting notify builds #####
mkdir "notify"
xcopy "backend\configuration" "notify\configuration" /e /h /c /i
xcopy "backend\templates" "notify\templates" /e /h /c /i

copy "backend\server\notify.ico" "build\notify.ico"
xcopy "build\*.*" "notify" /c
xcopy "build\windows\*.*" "notify" /c
copy "version.txt" "notify\version.txt"

IF NOT EXIST version.txt (
    ECHO ##### No version.txt found, defaulting to package version #####
    node -p "require('./frontend/package.json').version" > version.txt
)

ECHO ##### Prepare notify frontend #####
set /p APP_BUILD_VERSION=<version.txt
echo %APP_BUILD_VERSION%
cd "frontend" && yarn install --force --frozen-lockfile && yarn build-windows
if %errorlevel% neq 0 exit /b %errorlevel%

ECHO ##### Building notify backend #####
cd "..\backend" && cargo build --release --bin notify_service && copy "target\release\notify_service.exe" "..\notify\notify_service.exe"
if %errorlevel% neq 0 exit /b %errorlevel%