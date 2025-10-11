@echo off
REM Simple mover: moves server.exe to C:\Data\Apps\installed\
REM Usage: install-server.cmd [source-path]

SETLOCAL ENABLEDELAYEDEXPANSION

if "%~1"=="" (
  set "SRC=%~dp0..\target\release\server.exe"
) else (
  set "SRC=%~1"
)

set "DEST=C:\Data\Apps\installed\"

if not exist "%DEST%" (
  mkdir "%DEST%"
)

echo Moving "%SRC%" to "%DEST%"
move /Y "%SRC%" "%DEST%"

ENDLOCAL
