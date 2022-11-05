FROM rust:1.59 as build

WORKDIR /{{ project-name }}

COPY . .

# Build for release
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    --mount=type=cache,sharing=private,target=/{{ project-name }}/target \
    cargo build --release && \
    # Copy executable out of the cache so it is available in the final image.
    cp target/release/{{ project-name }} ./{{ project-name }}

# The final release image
FROM debian:bullseye-slim

ARG RUN_MODE=production
ENV TZ=Etc/UTC \
    RUN_MODE=${RUN_MODE} \
    RUST_LOG=info

EXPOSE 3000

COPY --from=build /{{ project-name }}/{{ project-name }} /usr/src/app/{{ project-name }}

# Install some Debian packages
RUN apt update && \
    apt-get -y install --no-install-recommends ca-certificates tzdata && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/* && \
    # Set up the app user
    groupadd {{ project-package }} && \
    useradd -g {{ project-package }} {{ project-package }} && \
    mkdir -p /usr/src/app && \
    chown -R {{ project-package }}:{{ project-package }} /usr/src/app

USER {{ project-package }}
WORKDIR /usr/src/app

COPY config config/

CMD ["./{{ project-name }}"]
