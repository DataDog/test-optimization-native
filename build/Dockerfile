# Stage 1: Build the static library
FROM golang:1.24.1-bookworm AS builder

# Install dependencies
RUN apt-get update && apt-get install -y gcc binutils

# Argument can be set during build time with --build-arg GOARCH=arm64
ARG GOARCH=amd64

# Configure environment variables for CGO, operating system, architecture, and compiler
ENV CGO_ENABLED=1 \
    GOOS=linux \
    GOARCH=$GOARCH \
    CC=gcc \
    CGO_CFLAGS="-O2 -Os -s -DNDEBUG -fdata-sections -ffunction-sections" \
    CGO_LDFLAGS="-s -Wl,--gc-sections"

WORKDIR /app

# Copy everything from the current directory to the PWD (Present Working Directory) inside the container
COPY . .

WORKDIR /app/internal/civisibility/native

# Build the library
RUN go build -tags civisibility_native -buildmode=c-archive -ldflags="-s -w" -gcflags="all=-l" -o ./output/static/libtestoptimization.a *.go
RUN strip --strip-unneeded ./output/static/libtestoptimization.a
RUN go build -tags civisibility_native -buildmode=c-shared -ldflags="-s -w" -gcflags="all=-l" -o ./output/dynamic/libtestoptimization.so *.go
RUN strip --strip-unneeded ./output/dynamic/libtestoptimization.so

# Stage 2: Extract the library
FROM alpine:latest

# Build arguments for the final archive names; you can override these during build
ARG FILE_NAME=libtestoptimization

# Install zip for compression
RUN apk add --no-cache zip

# Create the output folder
RUN mkdir -p /output

# Copy the static library and header from the builder stage
COPY --from=builder /app/internal/civisibility/native/output/static/libtestoptimization.a /output/static/libtestoptimization.a
COPY --from=builder /app/internal/civisibility/native/output/static/libtestoptimization.h /output/static/libtestoptimization.h
COPY --from=builder /app/internal/civisibility/native/output/dynamic/libtestoptimization.so /output/dynamic/libtestoptimization.so
COPY --from=builder /app/internal/civisibility/native/output/dynamic/libtestoptimization.h /output/dynamic/libtestoptimization.h

# Compress both files into zip archives
RUN cd /output/static && zip -j -9 ../${FILE_NAME}-static.zip *.*
RUN cd /output/dynamic && zip -j -9 ../${FILE_NAME}-dynamic.zip *.*
# Create a SHA256 checksum file for the archive
RUN sha256sum /output/${FILE_NAME}-static.zip > /output/${FILE_NAME}-static.zip.sha256sum
RUN sha256sum /output/${FILE_NAME}-dynamic.zip > /output/${FILE_NAME}-dynamic.zip.sha256sum

RUN rm -r /output/static /output/dynamic

# Command to run when the container starts
ENTRYPOINT ["sh", "-c", "ls /output && cp /output/*.* /libtestoptimization && echo 'Static library copied.'"]
