FROM rust:alpine AS build
WORKDIR /app
COPY Cargo.toml .
COPY Cargo.lock .
RUN cargo build --release

FROM postgres:alpine
WORKDIR /app
COPY --from=build /app/target/release/rust-api-server .
COPY prisma/schema.prisma ./prisma/schema.prisma
COPY docker-compose.yml ./docker-compose.yml
ENV DATABASE_URL=postgres://postgres:postgres@db:5432/postgres
CMD ["./rust-api-server"]
EXPOSE 8080