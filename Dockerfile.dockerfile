FROM ubuntu:22.04 as builder
RUN apt update && apt install -y build-essential bc bison flex libssl-dev git wget \
    curl musl-tools rustc cargo qemu-system-x86
WORKDIR /build
COPY . .
RUN make deps && make build
CMD ["make", "run-qemu"]