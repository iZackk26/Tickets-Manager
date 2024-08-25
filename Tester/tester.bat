@echo off
set times=5

for /l %%i in (1,1,%times%) do (
    "./Client.exe"
)