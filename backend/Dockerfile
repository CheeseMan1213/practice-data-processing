# # syntax=docker/dockerfile:1
# # q: What does the above line mean?
# # a: It specifies the syntax version of the Dockerfile. The version 1 syntax is
# #    the only syntax that supports the "mount" instruction.
# ARG RUST_VERSION=1.82.0
# ARG APP_NAME=backend

# ################################################################################
# # Create a stage for building the application.

# FROM rust:${RUST_VERSION} AS build
# ARG APP_NAME
# WORKDIR /app

# # RUN cargo generate-lockfile

# # Install host build dependencies.
# # RUN apk add --no-cache build-base cmake openssl-dev pkgconfig postgresql-dev
# RUN apt-get update && apt-get install -y build-essential cmake libpq-dev pkg-config libssl-dev

# ENV SQLX_OFFLINE=true

# # Build the application.
# # Leverage a cache mount to /usr/local/cargo/registry/
# # for downloaded dependencies, a cache mount to /usr/local/cargo/git/db
# # for git repository dependencies, and a cache mount to /app/target/ for
# # compiled dependencies which will speed up subsequent builds.
# # Leverage a bind mount to the src directory to avoid having to copy the
# # source code into the container. Once built, copy the executable to an
# # output directory before the cache mounted /app/target is unmounted.
# # COPY ../../Cargo.lock .
# RUN --mount=type=bind,source=src,target=src \
#     --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
#     --mount=type=bind,source=.sqlx,target=.sqlx \
#     --mount=type=cache,target=/app/target/ \
#     --mount=type=cache,target=/usr/local/cargo/git/db \
#     --mount=type=cache,target=/usr/local/cargo/registry/ \
# cargo build --release && \
# cp ./target/release/$APP_NAME /bin/server

# ################################################################################
# # Create a new stage for running the application that contains the minimal
# # runtime dependencies for the application. This often uses a different base
# # image from the build stage where the necessary files are copied from the build
# # stage.
# #
# # The example below uses the alpine image as the foundation for running the app.
# # By specifying the "3.18" tag, it will use version 3.18 of alpine. If
# # reproducability is important, consider using a digest
# # (e.g., alpine@sha256:664888ac9cfd28068e062c991ebcff4b4c7307dc8dd4df9e728bedde5c449d91).
# FROM rust:${RUST_VERSION} AS final

# # Create a non-privileged user that the app will run under.
# # See https://docs.docker.com/go/dockerfile-user-best-practices/
# ARG UID=10001
# ARG DATABASE_URL
# ENV DATABASE_URL=${DATABASE_URL}
# ENV SERVER_PORT='3002'
# ENV SERVER_ADDRESS='0.0.0.0'
# RUN adduser \
#     --disabled-password \
#     --gecos "" \
#     --home "/nonexistent" \
#     --shell "/sbin/nologin" \
#     --no-create-home \
#     --uid "${UID}" \
#     appuser
# USER appuser

# # Copy the executable from the "build" stage.
# COPY --from=build /bin/server /bin/

# # Expose the port that the application listens on.
# EXPOSE 3002

# # What the container should run when it is started.
# CMD ["/bin/server"]


##################################################
ARG RUST_VERSION=1.82.0
ARG APP_NAME=backend

FROM rust:${RUST_VERSION} AS build
ARG APP_NAME
ENV SQLX_OFFLINE=true
WORKDIR /app

RUN apt-get update && apt-get install -y build-essential cmake libpq-dev pkg-config libssl-dev

# Copy Cargo files first for better layer caching
COPY Cargo.toml ./

# Create a dummy main.rs to build dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release --manifest-path=Cargo.toml && rm -rf src

# Now copy the real source code
COPY ./src ./src/

# Build the actual application
RUN cargo build --release --manifest-path=Cargo.toml
RUN cp ./target/release/$APP_NAME /bin/server

FROM rust:${RUST_VERSION} AS final

ARG UID=10001
ARG DATABASE_URL
ENV DATABASE_URL=${DATABASE_URL}
ENV SERVER_PORT='3002'
ENV SERVER_ADDRESS='0.0.0.0'
RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    appuser
USER appuser

# COPY --from=build /app/target/release/my_app /app/my_app
# COPY --from=build /app/target/release/backend /backend/my_app
# cp ./target/release/$APP_NAME /bin/server
# COPY --from=build /app/target/release/$APP_NAME /bin/server
COPY --from=build /bin/server /bin/
EXPOSE 3002
# CMD ["/app/my_app"]
# CMD ["/backend/my_app"]
CMD ["/bin/server"]
