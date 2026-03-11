# ============================================
# Dockerfile Optimizado - Backend Rust
# ============================================

# Etapa 1: Construcción
# Usamos Rust 1.88 que soporta todas las dependencias modernas
FROM rust:1.88-bookworm as builder
WORKDIR /app/backend

# Instalar dependencias necesarias para la compilación
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copia solo Cargo.toml
COPY Cargo.toml ./

# Crea una estructura dummy para cachear las dependencias
# Cargo generará el Cargo.lock automáticamente
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release

# Limpia el código dummy
RUN rm -rf src

COPY src ./src
COPY migrations ./migrations

# Compila el código real
RUN cargo build --release

# Etapa 2: Runtime
FROM debian:bookworm-slim

# Instalar librerías necesarias y curl para healthcheck
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    libssl3 \
    ca-certificates \
    curl \
    && rm -rf /var/lib/apt/lists/*

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
