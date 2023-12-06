@ECHO ##### Removing installers folder #####
@rmdir "installers" /s /q

@ECHO ##### Update SUF files #####
copy /y notify\notify\notify_service.suf notify\notify_service.suf

@ECHO ##### Adjusting SUFS #####
SET installerWorkspace=%cd%\notify
SET SUFlocation=%installerWorkspace%\notify
FOR /F "delims=*" %%i in ('more notify\notify\version.txt') do SET versionTag=%%i
@ECHO "current tag = %versionTag%"
SET installersOutputFolder=%cd%\installers

@cd notify
node "%SUFlocation%\adjustSUFs.js"
@cd ..

@ECHO ##### Creating installers #####
start "" /wait "C:\Program Files (x86)\Setup Factory 9\SUFDesign.exe" /BUILD /LOG:installers\setup-factory.log "%installerWorkspace%\notify_service.suf"
