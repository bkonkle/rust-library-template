# {{project-name}}

[![Crates.io](https://img.shields.io/crates/v/{{project-name}}.svg)](https://crates.io/crates/{{project-name}})
[![Docs.rs](https://docs.rs/{{project-name}}/badge.svg)](https://docs.rs/{{project-name}})
[![CI](https://github.com/{{gh-username}}/{{project-name}}/workflows/CI/badge.svg)](https://github.com/{{gh-username}}/{{project-name}}/actions)
[![Coverage Status](https://coveralls.io/repos/github/{{gh-username}}/{{project-name}}/badge.svg?branch=main)](https://coveralls.io/github/{{gh-username}}/{{project-name}}?branch=main)

## Installation

### Cargo

- Install Rust and Cargo by following [this](https://www.rust-lang.org/tools/install) guide.
- Run `cargo install {{project-name}}`

## Development

To set up a development environment to build this project, you'll need to install some helpful tools.

### Clippy

For helpful linting rools, install [Clippy](https://github.com/rust-lang/rust-clippy)

Run it with `cargo`:

```sh
cargo clippy --fix
```

Configure the `rust-analyzer` VS Code plugin to use it (in _settings.json_):

```json
{
    "rust-analyzer.checkOnSave.command": "clippy"
}
```

{% if crate_type == "bin" %}### Running Docker

To run the docker-compose formation with just the supporting services needed to run `cargo make dev`:

```sh
cargo make docker up -d
```

To shut it down:

```sh
cargo make docker down
```

{% endif %}### Cargo Make

To build scripts from the _Makefile.toml_, install Cargo Make:

```sh
cargo install cargo-make
```

### Running the Local dev server

Use `cargo` to run the dev server locally:

```sh
cargo make dev
```

### Update Dependencies

First, install the `outdated` command for `cargo`:

```sh
cargo install --locked cargo-outdated
```

Then, update and check for any major dependency changes:

```sh
cargo update
cargo outdated
```

{% if crate_type == "bin" %}### Running Integration Tests

To integration test, you need to have the Docker Compose stack with Postgres and Redis running locally, or within your CI pipeline.

NOTE: This is destructive, and will wipe out data within your local database. See below for how to use an alternate test database locally.

To run the integration tests:

```sh
cargo make integration
```

## Deployment

### Building Docker Containers Locally

To build locally, use Buildkit:

```sh
DOCKER_BUILDKIT=1 docker build -t {{ project-name }} -f apps/api/Dockerfile .
```

To clear the build cache:

```sh
docker builder prune --filter type=exec.cachemount
```

To inspect the local filesystem:

```sh
docker run --rm -it --entrypoint=/bin/bash {{ project-name }}
```

To inspect the full build context:

```sh
docker image build --no-cache -t build-context -f - . <<EOF
FROM busybox
WORKDIR /build-context
COPY . .
CMD find .
EOF

docker container run --rm build-context
```

And to clean up the build context test image:

```sh
docker image rm build-context
```

{% endif %}## License

Licensed under the MIT license ([LICENSE](LICENSE) or <http://opensource.org/licenses/MIT>).

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

See [CONTRIBUTING.md](CONTRIBUTING.md).
