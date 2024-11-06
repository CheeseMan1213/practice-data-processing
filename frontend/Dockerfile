# ################################################################################
# # Create a stage for building the application.
# ARG RUST_VERSION=1.81.0
# ARG APP_NAME=frontend

# # Set the base image for the build stage using the slim version of the bullseye image.
# FROM rust:${RUST_VERSION} AS tooling

# RUN apt-get update && apt-get install -y \
#     pkg-config \
#     nodejs \
#     npm \
#     git \
#     # build-essential \
#     # wget \
#     # curl \
#     # libc6-dev \
#     # libssl-dev \
#     # cmake \
#     # make \
#     && rm -rf /var/lib/apt/lists/*

# RUN cargo install wasm-pack trunk

# # Install latest wasm-opt from source
# # RUN git clone https://github.com/WebAssembly/binaryen.git && \
# #     cd binaryen && \
# #     cmake . && \
# #     make && \
# #     make install && \
# #     cd .. && \
# #     rm -rf binaryen

# # Set environment variables
# # ENV RUSTUP_HOME=/usr/local/rustup \
# #     CARGO_HOME=/usr/local/cargo \
# #     PATH=/usr/local/cargo/bin:/usr/local/bin:$PATH \
# #     WASM_BINDGEN_USE_NO_MODULE=1

# FROM tooling AS build

# ARG APP_NAME
# WORKDIR /usr/src/app

# # Copy the backend Cargo.toml.
# COPY Cargo.toml ./Cargo.toml

# # Create a dummy main.rs to build dependencies.
# RUN mkdir src && \
#   echo "fn main() {}" > src/main.rs && \
#   cargo build --release

# # Copy the source code into the container
# COPY src src
# COPY index.html index.html

# # Build the real application.
# RUN rustup target add wasm32-unknown-unknown
# # RUN wasm-pack build --release --target web
# RUN trunk build --release

# ################################################################################
# # Create a new stage for running the application using a slim version of the bullseye image.
# # FROM nginx:alpine-slim AS final
# FROM nginx:stable-bullseye AS final

# ARG APP_NAME
# # I needed to make this an env var here because ARG does not stay after build time. ENV does.
# ENV APP_NAME=${APP_NAME}

# # Create a non-root user to run the application.
# ARG UID=10001
# RUN adduser \
#     --disabled-password \
#     --gecos "" \
#     --home "/nonexistent" \
#     --shell "/sbin/nologin" \
#     --no-create-home \
#     --uid "${UID}" \
#     appuser

# # Copy the compiled binary from the build stage
# # COPY --from=build /app/target/wasm32-unknown-unknown/release/${APP_NAME}.wasm /usr/local/bin/${APP_NAME}
# COPY --from=build /usr/src//app/dist /usr/share/nginx/html
# RUN chown appuser:appuser /usr/local/bin/${APP_NAME}
# USER appuser

# EXPOSE 8082

# # Run the application.
# CMD ["nginx", "-g", "daemon off;"]

# ################################################################################
# Trying without trunk build --release. Build locally and just copy the dist folder to nginx.
# Use a lightweight base image
FROM nginx:alpine

# Copy the built frontend files to the container
COPY dist /usr/share/nginx/html
# COPY nginx.conf /etc/nginx/nginx.conf
COPY nginx.conf /etc/nginx/conf.d/default.conf

# Expose port 8082
EXPOSE 8082

# Start Nginx
CMD ["nginx", "-g", "daemon off;"]