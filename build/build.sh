#!/bin/bash

# Exit immediately if a command exits with a non-zero status
set -e

mkdir -p ./output

echo "Building the library for macos"
#it requires zip (usually pre-installed on macOS)

export CGO_CFLAGS="-mmacosx-version-min=11.0 -O2 -Os -DNDEBUG -fdata-sections -ffunction-sections"
export CGO_LDFLAGS="-Wl,-x"
export GOOS=darwin
export CGO_ENABLED=1
GOARCH=arm64 go build -tags civisibility_native -buildmode=c-archive -ldflags="-s -w" -gcflags="all=-l" -o ./output/macos-arm64-libtestoptimization-static/libtestoptimization.a *.go
GOARCH=arm64 go build -tags civisibility_native -buildmode=c-shared -ldflags="-s -w" -gcflags="all=-l" -o ./output/macos-arm64-libtestoptimization-dynamic/libtestoptimization.dylib *.go
GOARCH=amd64 go build -tags civisibility_native -buildmode=c-archive -ldflags="-s -w" -gcflags="all=-l" -o ./output/macos-x64-libtestoptimization-static/libtestoptimization.a *.go
GOARCH=amd64 go build -tags civisibility_native -buildmode=c-shared -ldflags="-s -w" -gcflags="all=-l" -o ./output/macos-x64-libtestoptimization-dynamic/libtestoptimization.dylib *.go
mkdir -p ./output/macos-libtestoptimization-static
lipo -create ./output/macos-arm64-libtestoptimization-static/libtestoptimization.a ./output/macos-x64-libtestoptimization-static/libtestoptimization.a -output ./output/macos-libtestoptimization-static/libtestoptimization.a
cp ./output/macos-arm64-libtestoptimization-static/libtestoptimization.h ./output/macos-libtestoptimization-static/libtestoptimization.h
zip -j -9 ./output/macos-libtestoptimization-static.zip ./output/macos-libtestoptimization-static/*.*
sha256sum ./output/macos-libtestoptimization-static.zip > ./output/macos-libtestoptimization-static.zip.sha256sum
rm -r ./output/macos-libtestoptimization-static
mkdir -p ./output/macos-libtestoptimization-dynamic
lipo -create ./output/macos-arm64-libtestoptimization-dynamic/libtestoptimization.dylib ./output/macos-x64-libtestoptimization-dynamic/libtestoptimization.dylib -output ./output/macos-libtestoptimization-dynamic/libtestoptimization.dylib
cp ./output/macos-arm64-libtestoptimization-dynamic/libtestoptimization.h ./output/macos-libtestoptimization-dynamic/libtestoptimization.h
zip -j -9 ./output/macos-libtestoptimization-dynamic.zip ./output/macos-libtestoptimization-dynamic/*.*
sha256sum ./output/macos-libtestoptimization-dynamic.zip > ./output/macos-libtestoptimization-dynamic.zip.sha256sum
rm -r ./output/macos-libtestoptimization-dynamic
rm -r ./output/macos-arm64-libtestoptimization-static ./output/macos-arm64-libtestoptimization-dynamic ./output/macos-x64-libtestoptimization-static ./output/macos-x64-libtestoptimization-dynamic

echo "Building the static library for linux-arm64"
docker build --platform linux/arm64 --build-arg GOARCH=arm64 --build-arg FILE_NAME=linux-arm64-libtestoptimization -t libtestoptimization-builder:arm64 -f ./Dockerfile ../../..
docker run --rm -v ./output:/libtestoptimization libtestoptimization-builder:arm64

echo "Building the static library for linux-x64"
docker build --platform linux/amd64 --build-arg GOARCH=amd64 --build-arg FILE_NAME=linux-x64-libtestoptimization -t libtestoptimization-builder:amd64 -f ./Dockerfile ../../..
docker run --rm -v ./output:/libtestoptimization libtestoptimization-builder:amd64

echo "Building the dynamic library for android-arm64"
docker build --platform linux/amd64 --build-arg GOARCH=arm64 --build-arg FILE_NAME=android-arm64-libtestoptimization -t libtestoptimization-builder:androidarm64 -f ./Dockerfile-android ../../..
docker run --rm -v ./output:/libtestoptimization libtestoptimization-builder:androidarm64

echo "Building the static library for ios"
GOOS=ios GOARCH=arm64 go build -tags civisibility_native -buildmode=c-archive -ldflags="-s -w" -gcflags="all=-l" -o ./output/ios-libtestoptimization-static/libtestoptimization.a *.go
zip -j -9 ./output/ios-libtestoptimization-static.zip ./output/ios-libtestoptimization-static/*.*
sha256sum ./output/ios-libtestoptimization-static.zip > ./output/ios-libtestoptimization-static.zip.sha256sum
rm -r ./output/ios-libtestoptimization-static

echo "done."