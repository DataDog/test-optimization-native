@echo off
setlocal
set GOOS=windows
set GOARCH=amd64
set CC=x:\mingw64\bin\x86_64-w64-mingw32-gcc.exe
set CGO_ENABLED=1
set CGO_CPPFLAGS=
set CGO_CXXFLAGS=-O2
set CGO_FFLAGS=-O2
set CGO_CFLAGS=-O2 -fno-unwind-tables -fno-asynchronous-unwind-tables
set CGO_LDFLAGS=-Wl,--no-seh

echo Building windows static library
go build -tags civisibility_native -buildmode=c-archive -ldflags "-linkmode=external -extldflags=-Wl,--no-seh" -o ./output/windows-x64-libtestoptimization-static/testoptimization.lib exports.go main.go

echo Stripping .pdata and .xdata sections from the static library
pushd ./output/windows-x64-libtestoptimization-static
set "AR=x:\mingw64\bin\llvm-ar.exe"
set "OBJCOPY=x:\mingw64\bin\llvm-objcopy.exe"

%AR% x testoptimization.lib
for %%f in (*.o) do (
    echo     stripping %%f
    %OBJCOPY% --remove-section=.pdata --remove-section=.xdata %%f
)
del testoptimization.lib
%AR% rc testoptimization.lib *.o
%AR% s  testoptimization.lib
popd

echo Building windows shared library
go build -tags civisibility_native -buildmode=c-shared -ldflags="-s -w" -o ./output/windows-x64-libtestoptimization-dynamic/testoptimization.dll exports.go main.go

docker build --platform linux/amd64 --build-arg GOARCH=amd64 --build-arg FILE_NAME=windows-x64-libtestoptimization -t win-libtestoptimization-builder:amd64 -f ./Dockerfile-windows .
docker run --rm --mount type=bind,src=.\output,dst=/libtestoptimization win-libtestoptimization-builder:amd64

endlocal