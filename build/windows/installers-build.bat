@ECHO ##### Removing installers folder #####
@rmdir "installers" /s /q

@ECHO ##### Adjusting SUFS #####
SET installerWorkspace=C:\Program Files (x86)\Jenkins\jobs\notifyMain - Installers\workspace\notify
SET SUFlocation=%installerWorkspace%\notify
FOR /F "delims=*" %%i in ('more notify\version.txt') do SET versionTag=%%i
@ECHO "current tag = %versionTag%"
SET installersOutputFolder=%WORKSPACE%\installers

@cd notify
node "%SUFlocation%\adjustSUFs.js"
@cd ..

@ECHO ##### Creating installers #####
start "" /wait "C:\Program Files (x86)\Setup Factory 9\SUFDesign.exe" /BUILD /LOG:installers\setup-factory.log "%installerWorkspace%\notify_service.suf"
