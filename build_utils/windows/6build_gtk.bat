mkdir C:\Python313
for /f "delims=" %%i in ('where python') do copy "%%i" "C:\Python313\python.exe" /Y
%USERPROFILE%\.local\bin\gvsbuild build gtk4
