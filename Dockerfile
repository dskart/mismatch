FROM node:20-bookworm-slim AS ui-builder

WORKDIR /usr/src/app/mismatch/src/ui
COPY ./src/ui .

RUN apt-get update && apt-get install -y \
    make \
    wget \ 
    && rm -rf /var/lib/apt/lists/*

RUN make update-htmx
RUN npm install
RUN npm run build

FROM debian:bookworm-slim AS models-builder

RUN apt-get update && apt-get install -y \
    make \
    wget \ 
    && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/app/mismatch/
COPY ./Makefile ./Makefile
RUN make models

FROM rust:1.84-slim-bookworm AS rust-builder

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    build-essential \
    wget \
    && rm -rf /var/lib/apt/lists/*

# Install ONNX Runtime
RUN wget https://github.com/microsoft/onnxruntime/releases/download/v1.16.3/onnxruntime-linux-x64-1.16.3.tgz \
    && tar -xzf onnxruntime-linux-x64-1.16.3.tgz \
    && cp onnxruntime-linux-x64-1.16.3/lib/libonnxruntime.so* /usr/lib/ \
    && cp -r onnxruntime-linux-x64-1.16.3/include/* /usr/include/ \
    && rm -rf onnxruntime-linux-x64-1.16.3*

WORKDIR /usr/src/app
RUN cargo new --bin mismatch
WORKDIR /usr/src/app/mismatch

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# Cache dependencies
RUN cargo build --release
RUN rm src/*.rs

COPY --from=ui-builder /usr/src/app/mismatch/src/ui ./src/ui
COPY ./src ./src

# Build the project
RUN rm ./target/release/deps/mismatch*
COPY ./Makefile ./Makefile
RUN cargo build --release
RUN make gen_word_dict

FROM debian:bookworm-slim

COPY --from=rust-builder /usr/lib/libonnxruntime.so* /usr/lib/
COPY --from=models-builder /usr/src/app/mismatch/models /usr/src/app/mismatch/models
COPY --from=rust-builder /usr/src/app/mismatch/src/ui/public /usr/src/app/mismatch/src/ui/public
COPY --from=rust-builder /usr/src/app/mismatch/target/release/mismatch /usr/local/bin/
COPY --from=rust-builder /usr/src/app/mismatch/gen /usr/src/app/mismatch/gen
ENV LD_LIBRARY_PATH=/usr/lib

RUN mismatch --help > /dev/null

ENV MISMATCH__APP__MODEL=potion-base-8M
ENV MISMATCH__APP__STORE__BLOB_STORAGE=LOCAL
EXPOSE 8080
ENTRYPOINT ["mismatch"]
CMD ["serve", "--port", "8080"]
