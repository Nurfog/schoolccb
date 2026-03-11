# ============================================
# Dockerfile Optimizado - Backend Rust
# ============================================

# Etapa 1: Construcción
FROM rust:1.77-slim-bookworm as builder
WORKDIR /app/backend

# Instalar dependencias necesarias para la compilación
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

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

# Etapa 2: Runtime con imagen minimalista
FROM gcr.io/distroless/cc-debian12

# Copiar certificados SSL para conexiones HTTPS
COPY --from=debian:bookworm-slim /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/

WORKDIR /app

# Copia los binarios compilados
COPY --from=builder /app/backend/target/release/colegio-backend /app/colegio-backend

# Configurar variables de entorno
ENV RUST_LOG=info
ENV PORT=8080

# Exponer el puerto
EXPOSE 8080

# Comando de inicio
CMD ["./colegio-backend"]
