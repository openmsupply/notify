IF NOT EXIST version.txt (
    @ECHO ##### No version.txt found, defaulting to package version #####
    node -p "require('./frontend/package.json').version" > version.txt
)
set /p APP_BUILD_VERSION=<version.txt
@ECHO %APP_BUILD_VERSION%
cd frontend && yarn install --force --frozen-lockfile && yarn build-windows
if %errorlevel% neq 0 exit /b %errorlevel%

waitfor /s %computername% /si frontend-build
