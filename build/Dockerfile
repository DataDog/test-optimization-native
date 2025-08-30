# Use an Alpine-based Go image for the builder stage
FROM datadog/libddwaf:toolchain AS builder

# Variables de entorno para el build universal
ENV QUIC_LTTng=0 \
    IsAlpine=true \
    AsUniversal=true

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

RUN ln -sf "$(which clang-16)" /usr/bin/clang && \
    ln -sf "$(which clang++-16)" /usr/bin/clang++

ENV CXX=clang++ \
    CC=clang

# Argument can be set during build time with --build-arg GOARCH=arm64
ARG GOARCH=amd64
ENV GOARCH=$GOARCH

# Definimos el MUSL target y comprobamos la arquitectura
# Guardamos el target en un archivo para usarlo en los comandos posteriores
RUN case "$GOARCH" in \
        amd64) echo "x86_64-none-linux-musl" > /musl_target ;; \
        arm64) echo "aarch64-none-linux-musl" > /musl_target ;; \
        *) echo "Unsupported architecture: $GOARCH" && exit 1 ;; \
    esac

# Definimos el intérprete de carga dinámica según la arquitectura
RUN case "$GOARCH" in \
        amd64) echo "ld-musl-x86_64.so.1" > /interpreter ;; \
        arm64) echo "ld-musl-aarch64.so.1" > /interpreter ;; \
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
    GOOS=linux 

WORKDIR /app
COPY . .

WORKDIR /app/internal/civisibility/native

# Comunes: Leemos el MUSL target y el intérprete definidos previamente
# Se usarán en ambos builds (estático y compartido)
RUN export MUSL_TARGET=$(cat /musl_target) && \
    export SYSROOT=/sysroot/$MUSL_TARGET && \
    export INTERPRETER=$(cat /interpreter) && \
    echo "Using SYSROOT: $SYSROOT and INTERPRETER: $INTERPRETER" && \
    mkdir -p ./output/static ./output/dynamic

# Build de la librería estática (.a) con buildmode=c-archive
RUN export MUSL_TARGET=$(cat /musl_target) && \
    export SYSROOT=/sysroot/$MUSL_TARGET && \
    export INTERPRETER=$(cat /interpreter) && \
    export CFLAGS="--target=$MUSL_TARGET -nostdinc++ -isystem $SYSROOT/usr/include \
      -Qunused-arguments -fPIC -rtlib=compiler-rt -unwindlib=libunwind \
      -static-libgcc -fno-omit-frame-pointer -ffunction-sections -fdata-sections" && \
    export LDFLAGS="--target=$MUSL_TARGET -fPIC -fuse-ld=lld-16 -nodefaultlibs \
      -L$SYSROOT/usr/lib -Wl,-Bstatic -lc++ -lc++abi ${SYSROOT}/usr/lib/libclang_rt.builtins.a -lunwind \
      -Wl,-Bdynamic -lc ${SYSROOT}/usr/lib/libclang_rt.builtins.a \
      -Wl,--dynamic-linker,$SYSROOT/lib/${INTERPRETER} -Wl,-rpath,$SYSROOT \
      -resource-dir $SYSROOT/usr/lib/resource_dir -Wl,--gc-sections -Wl,--discard-all -Wl,--icf=safe" && \
    export CGO_CFLAGS="$CFLAGS" && \
    export CGO_LDFLAGS="$LDFLAGS" && \
    go build -tags civisibility_native -buildmode=c-archive -o ./output/static/libtestoptimization.a *.go

# Build de la librería compartida (.so) con buildmode=c-shared
RUN export MUSL_TARGET=$(cat /musl_target) && \
    export SYSROOT=/sysroot/$MUSL_TARGET && \
    export INTERPRETER=$(cat /interpreter) && \
    export CFLAGS="--target=$MUSL_TARGET -nostdinc++ -isystem $SYSROOT/usr/include \
      -Qunused-arguments -fPIC -rtlib=compiler-rt -unwindlib=libunwind \
      -static-libgcc -fno-omit-frame-pointer -ffunction-sections -fdata-sections" && \
    export LDFLAGS="--target=$MUSL_TARGET -fPIC -fuse-ld=lld-16 -nodefaultlibs \
      -L$SYSROOT/usr/lib -Wl,-Bstatic -lc++ -lc++abi ${SYSROOT}/usr/lib/libclang_rt.builtins.a -lunwind \
      -Wl,-Bstatic -lc -Wl,-Bdynamic \
      -resource-dir $SYSROOT/usr/lib/resource_dir -Wl,--gc-sections -Wl,--discard-all -Wl,--icf=safe" && \
    export CGO_CFLAGS="$CFLAGS" && \
    export CGO_LDFLAGS="$LDFLAGS" && \
    go build -tags civisibility_native -buildmode=c-shared -o ./output/dynamic/libtestoptimization.so *.go


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
RUN cd /output/static && zip -j -9 ../${FILE_NAME}-static.zip *.* && \
    cd /output/dynamic && zip -j -9 ../${FILE_NAME}-dynamic.zip *.*

# Create a SHA256 checksum file for the archive
RUN sha256sum /output/${FILE_NAME}-static.zip > /output/${FILE_NAME}-static.zip.sha256sum && \
    sha256sum /output/${FILE_NAME}-dynamic.zip > /output/${FILE_NAME}-dynamic.zip.sha256sum

RUN rm -r /output/static /output/dynamic

# Command to run when the container starts
ENTRYPOINT ["sh", "-c", "ls /output && cp /output/*.* /libtestoptimization && echo 'Static library copied.'"]
