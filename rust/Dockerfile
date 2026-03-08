# Etapa 1: Construcción
FROM rust:latest as builder
WORKDIR /app/backend

# Copia Cargo.toml y Cargo.lock (si existe)
COPY Cargo.toml ./
COPY Cargo.lock* ./

# Crea una estructura dummy para cachear las dependencias
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release

# Limpia y copia el código real
RUN rm -rf src && \
    rm -rf target/release/deps/colegio_backend*

COPY src ./src
COPY migrations ./migrations

# Compila el código real
RUN cargo build --release

# Etapa 2: Runtime
FROM debian:bookworm-slim

# Instala libpq (dependencia para PostgreSQL)
RUN apt-get update && \
    apt-get install -y libpq5 ca-certificates && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copia los binarios compilados
COPY --from=builder /app/backend/target/release/colegio-backend /app/colegio-backend
COPY --from=builder /app/backend/target/release/bootstrap /app/bootstrap

# Expone el puerto
EXPOSE 8080

# Comando de inicio
CMD ["./colegio-backend"]
