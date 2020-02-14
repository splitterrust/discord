FROM rust:1.41 as builder

WORKDIR /usr/src/

# Create blank project
RUN USER=root cargo new splitterrust_discord

# Install deps, this will cache them so we can
# build the container faster for development.
COPY Cargo.toml Cargo.lock /usr/src/splitterrust_discord/

#RUN apt-get update && \
#    apt-get install -y libpq5

WORKDIR /usr/src/splitterrust_discord

# Remove last line, which is our local library (for the moment)
RUN sed -i '$ d' Cargo.toml

RUN cargo build --release

# Now copy source files and install the application.
COPY . .
RUN cargo install --path .

# Build container with only the build package for a
# smaller image.
FROM debian:buster-slim

# Create user and install additional deps
RUN groupadd -r splitterrust && useradd -r -s /bin/false -g splitterrust splitterrust
RUN apt-get update && apt-get install -y libpq5

ENV DISCORD /usr/local/bin/splitterrust_discord

COPY --from=builder /usr/local/cargo/bin/splitterrust_discord $DISCORD
COPY docker_entrypoint.sh /usr/local/bin/

RUN ln -s /usr/local/bin/docker_entrypoint.sh / # backwards compat

ENTRYPOINT ["docker_entrypoint.sh"]

CMD ["splitterrust_discord"]
