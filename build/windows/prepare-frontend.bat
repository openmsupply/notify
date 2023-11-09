@ECHO ##### Prepare notify frontend #####
set /p APP_BUILD_VERSION=<version.txt
echo %APP_BUILD_VERSION%
cd "frontend" && yarn install --force --frozen-lockfile && yarn build-windows
if %errorlevel% neq 0 exit /b %errorlevel%