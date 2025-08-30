##NOTE: I like being able to run docker build without neeidng to be in different directories. So, I use the -f flag to specify the Dockerfile location.
# and run my build from the root of the project. Example: docker build -f backend/Dockerfile .
# Therefore the paths in the Dockerfile are relative to the root of the project.

################################################################################
# Create a stage for building the application.
ARG RUST_VERSION=1.89.0
ARG APP_NAME=backend

# Set the base image for the build stage using the slim version of the bullseye image.
FROM rust:${RUST_VERSION}-slim-bullseye AS build

# Install essential build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    build-essential \
    && rm -rf /var/lib/apt/lists/*

ARG APP_NAME
WORKDIR /app

# Copy the backend Cargo.toml and Cargo.lock.
COPY backend/Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock

# Create a dummy main.rs to build dependencies.
RUN mkdir src && \
  echo "fn main() {}" > src/main.rs && \
  cargo build

# Set environment variable for SQLx so we can build offline.
# Needed to do cargo sqlx prepare on my host, so I could copy the .sqlx directory into the container.
ENV SQLX_OFFLINE=true

# Copy the source code into the container
RUN rm src/main.rs
COPY backend/.sqlx .sqlx
COPY backend/src src

# Build the real application.
RUN cargo build --release

################################################################################
# Create a new stage for running the application using a slim version of the bullseye image.
FROM debian:bullseye-slim AS final

# Install essential build dependencies
RUN apt-get update && apt-get install -y \
    iputils-ping \ 
    ca-certificates \
    libssl1.1 \
    && rm -rf /var/lib/apt/lists/*

ARG APP_NAME
# I needed to make this an env var here bacause ARG does not stay after build time. ENV does.
ENV APP_NAME=${APP_NAME}

# Right now the expectaton is that the database URL will be passed in as with the docker build command.
# Example: docker build --build-arg DATABASE_URL={{actual database url}} -t pdp_backend_docker_image .
ARG DATABASE_URL
ENV DATABASE_URL=${DATABASE_URL}
ENV SERVER_PORT='3002'
ENV SERVER_ADDRESS='0.0.0.0'

# Create a non-root user to run the application.
ARG UID=10001
RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    appuser

# Copy the compiled binary from the build stage
COPY --from=build /app/target/release/${APP_NAME} /usr/local/bin/${APP_NAME}
RUN chown appuser:appuser /usr/local/bin/${APP_NAME}
USER appuser

# Expose port 3002
EXPOSE 3002

# Run the application.
CMD ["sh", "-c", "/usr/local/bin/${APP_NAME}"]
