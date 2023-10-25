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
cd ../backend && cargo build --release && cargo build --release --bin notify_service
if %errorlevel% neq 0 exit /b %errorlevel%