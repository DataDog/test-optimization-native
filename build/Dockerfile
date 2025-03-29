# Use an Alpine-based Go image for the builder stage
FROM atadog/libddwaf:toolchain AS builder

ENV QUIC_LTTng=0

RUN apt-get update \
    && apt-get -y upgrade \
    && DEBIAN_FRONTEND=noninteractive apt-get install -y --fix-missing \
      cmake \
      git \
      wget \
      curl \
      cmake \
      make \
      gcc \
      build-essential \
      uuid-dev \
      autoconf \
      gdb \
      tar \
      \
    && rm -rf /var/lib/apt/lists/*

# We need to set this environment variable to make sure
# that compiling/linking will act as if it was on Alpine
ENV IsAlpine=true \
## This dockerfile is meant to build universal binaries
    AsUniversal=true


RUN ln -s `which clang-16` /usr/bin/clang  && \
    ln -s `which clang++-16` /usr/bin/clang++

ENV \
    CXX=clang++ \
    CC=clang

# Argument can be set during build time with --build-arg GOARCH=arm64
ARG GOARCH=amd64

# Download and extract Go
ENV GOFILENAME=go1.24.1.linux-${GOARCH}.tar.gz \
    GODOWNLOADURL=https://go.dev/dl/go1.24.1.linux-${GOARCH}.tar.gz
RUN wget -O go.tar.gz ${GODOWNLOADURL} && \
    tar -C /usr/local -xzf go.tar.gz && \
    rm go.tar.gz
ENV PATH="$PATH:/usr/local/go/bin"

# Configure environment variables for CGO, operating system, architecture, and compiler
ENV CGO_ENABLED=1 \
    GOOS=linux \
    GOARCH=$GOARCH

# Target para musl según arquitectura
RUN echo "GOARCH=$GOARCH" \
 && if [ "$GOARCH" = "amd64" ]; then \
        export MUSL_TARGET="x86_64-linux-musl"; \
    elif [ "$GOARCH" = "arm64" ]; then \
        export MUSL_TARGET="aarch64-linux-musl"; \
    else \
        echo "Unsupported architecture: $GOARCH" && exit 1; \
    fi \
 && echo "MUSL_TARGET=$MUSL_TARGET" > /musl_target_env.sh

WORKDIR /app

# Copy everything from the current directory to the PWD (Present Working Directory) inside the container
COPY . .

WORKDIR /app/internal/civisibility/native

# Build the library
# Ejecutamos el build en una capa aparte para soportar $MUSL_TARGET
RUN . /musl_target_env.sh \
 && export CGO_CFLAGS="--target=$MUSL_TARGET -nostdinc++ -isystem /toolchain/include" \
 && export CGO_LDFLAGS="--target=$MUSL_TARGET -L/toolchain/lib -lc++ -lc++abi -lunwind -lm" \
 && go build -tags civisibility_native -buildmode=c-archive -o ./output/static/libtestoptimization.a *.go
RUN strip --strip-unneeded ./output/static/libtestoptimization.a

RUN . /musl_target_env.sh \
 && export CGO_CFLAGS="--target=$MUSL_TARGET -nostdinc++ -isystem /toolchain/include" \
 && export CGO_LDFLAGS="--target=$MUSL_TARGET -L/toolchain/lib -lc++ -lc++abi -lunwind -lm" \
 && go build -tags civisibility_native -buildmode=c-shared -o ./output/dynamic/libtestoptimization.so *.go
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
