####################################################################################################
## Builder
####################################################################################################
FROM rust:1.69.0 AS builder

RUN update-ca-certificates

ENV USER=arkaitzserv
ENV UID=10001

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"


WORKDIR /

COPY ./ .

RUN cargo build --release

####################################################################################################
## Final image
####################################################################################################
FROM gcr.io/distroless/cc

# Import from builder.
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

WORKDIR /

# Copy our build
COPY --from=builder /target/release/arkaitzserv ./

# Use an unprivileged user.
USER arkaitzserv:arkaitzserv

RUN mkdir /public && chown arkaitzserv:arkaitzserv /public

ARG ROOT_DIR
ARG API_URL
ARG ASSETS_DIR

CMD ["/arkaitzserv"]