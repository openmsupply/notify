@ECHO ##### Starting notify builds #####
mkdir "notify"
xcopy "notify\configuration" "notify\configuration" /e /h /c /i

copy "backend\server\notify.ico" "build\notify.ico"
xcopy "build\*.*" "notify" /c
xcopy "build\windows\*.*" "notify" /c
copy "version.txt" "notify\version.txt"

@ECHO ##### Prepare notify frontend #####
start /wait /b build\windows\notify-prepare.bat
@if %errorlevel% neq 0 exit /b %errorlevel%
waitfor /t 60 notify-preparation

ECHO ##### Building notify app #####
cd backend && cargo build --release --bin notify_service && copy "target\release\notify_service.exe" "..\notify\notify_service.exe"
if %errorlevel% neq 0 exit /b %errorlevel%