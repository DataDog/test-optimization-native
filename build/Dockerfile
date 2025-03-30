# Use an Alpine-based Go image for the builder stage
FROM datadog/libddwaf:toolchain AS builder

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
# Set MUSL_TARGET depending on GOARCH
ARG MUSL_TARGET
ENV GOARCH=$GOARCH

# Set MUSL target and interpreter (for legacy references, even if we don't use it anymore)
RUN case "$GOARCH" in \
        amd64) echo "x86_64-none-linux-musl" > /musl_target ;; \
        arm64) echo "aarch64-none-linux-musl" > /musl_target ;; \
        *) echo "Unsupported architecture: $GOARCH" && exit 1 ;; \
    esac

# Install Go manually
ENV GOFILENAME=go1.24.1.linux-${GOARCH}.tar.gz \
    GODOWNLOADURL=https://go.dev/dl/go1.24.1.linux-${GOARCH}.tar.gz

RUN wget -O go.tar.gz ${GODOWNLOADURL} && \
    tar -C /usr/local -xzf go.tar.gz && \
    rm go.tar.gz

ENV PATH="$PATH:/usr/local/go/bin" \
    CGO_ENABLED=1 \
    GOOS=linux \
    GOARCH=$GOARCH

WORKDIR /app
COPY . .

WORKDIR /app/internal/civisibility/native

# Build static library (.a)
RUN export MUSL_TARGET=$(cat /musl_target) && \
    export SYSROOT=/sysroot/$MUSL_TARGET && \
    export CFLAGS="--target=$MUSL_TARGET -nostdinc++ -isystem $SYSROOT/usr/include \
      -Qunused-arguments -fPIC -rtlib=compiler-rt -unwindlib=libunwind \
      -static-libgcc -fno-omit-frame-pointer -ffunction-sections -fdata-sections" && \
    export LDFLAGS="--target=$MUSL_TARGET -fPIC -fuse-ld=lld-16 -nodefaultlibs \
      -L$SYSROOT/usr/lib -Wl,-Bstatic -lc++ -lc++abi -lunwind \
      -Wl,-Bdynamic -lc \
      -Wl,-rpath=$SYSROOT \
      -resource-dir $SYSROOT/usr/lib/resource_dir \
      -Wl,--gc-sections -Wl,--discard-all -Wl,--icf=safe" && \
    export CGO_CFLAGS="$CFLAGS" && \
    export CGO_LDFLAGS="$LDFLAGS" && \
    mkdir -p ./output/static && \
    go build -tags civisibility_native -buildmode=c-archive -o ./output/static/libtestoptimization.a *.go && \

# Build shared library (.so)
RUN export MUSL_TARGET=$(cat /musl_target) && \
    export SYSROOT=/sysroot/$MUSL_TARGET && \
    export CFLAGS="--target=$MUSL_TARGET -nostdinc++ -isystem $SYSROOT/usr/include \
      -Qunused-arguments -fPIC -rtlib=compiler-rt -unwindlib=libunwind \
      -static-libgcc -fno-omit-frame-pointer -ffunction-sections -fdata-sections" && \
    export LDFLAGS="--target=$MUSL_TARGET -fPIC -fuse-ld=lld-16 -nodefaultlibs \
      -L$SYSROOT/usr/lib -Wl,-Bstatic -lc++ -lc++abi -lunwind \
      -Wl,-Bdynamic -lc \
      -Wl,-rpath=$SYSROOT \
      -resource-dir $SYSROOT/usr/lib/resource_dir \
      -Wl,--gc-sections -Wl,--discard-all -Wl,--icf=safe" && \
    export CGO_CFLAGS="$CFLAGS" && \
    export CGO_LDFLAGS="$LDFLAGS" && \
    mkdir -p ./output/dynamic && \
    go build -tags civisibility_native -buildmode=c-shared -o ./output/dynamic/libtestoptimization.so *.go && \


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
