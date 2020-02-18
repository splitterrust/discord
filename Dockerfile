FROM rust:1.41 as builder

RUN cargo install splitterrust_discord

# The smaller image with only the binary and all needed deps
FROM debian:buster-slim
RUN groupadd -r splitterrust && useradd -r -s /bin/false -g splitterrust splitterrust
RUN apt-get update && apt-get install -y libpq5

ENV DISCORD /usr/local/cargo/bin/splitterrust_discord

COPY --from=builder $DISCORD $DISCORD
COPY docker_entrypoint.sh /usr/local/bin/

RUN ln -s /usr/local/bin/docker_entrypoint.sh / # backwards compat

EXPOSE 8088

ENTRYPOINT ["docker_entrypoint.sh"]

CMD ["splitterrust_discord"]
