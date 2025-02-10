# Build

## Linting & Formatting

Linting and formatting are handled by the `clippy` and `rustfmt` tools during the build process.
There is no need to run them separately.

## Unit Testiing

Unit testing is handled by the `cargo test` command.
This is run automatically during the build process. There is no need to run it separately.

## Build the project with Docker Compose

```bash
docker compose build
```

Remember that you can build the Rust API alone with the following command:

```bash
docker compose build circuit
```

Then, you can deploy the Docker image to a server in production.

Alternativelly, you can run the [build.sh](build.sh) script to build the project.
