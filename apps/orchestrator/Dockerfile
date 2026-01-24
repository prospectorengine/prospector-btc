# =================================================================
# APARATO: ORCHESTRATOR HYPER-BUILDER (V17.0 - SHARDING READY)
# CLASIFICACIÓN: OPS INFRASTRUCTURE
# RESPONSABILIDAD: COMPILACIÓN RUST NIGHTLY Y ENTORNO DE EJECUCIÓN
# =================================================================

# --- ETAPA 1: BASE DE INGENIERÍA (CHEF) ---
# Usamos la imagen oficial basada en Debian 12 (Bookworm) para estabilidad
FROM rust:bookworm AS chef

# 1. Instalación de herramientas de sistema necesarias para compilación
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# 2. TRANSICIÓN A NIGHTLY (CRÍTICO PARA FEATURES 2024 Y SIMD)
# Instalamos y configuramos nightly como el toolchain por defecto
RUN rustup toolchain install nightly --profile minimal --allow-downgrade && \
    rustup default nightly

# 3. Instalación de cargo-chef para optimización de caché de capas
RUN cargo install cargo-chef
WORKDIR /app

# --- ETAPA 2: PLANNER (CÁLCULO DE DEPENDENCIAS) ---
# Generamos el archivo de receta (recipe.json) para cachear dependencias
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# --- ETAPA 3: BUILDER (COMPILACIÓN ATÓMICA) ---
# Compilamos el binario final aprovechando el caché de cargo-chef
FROM chef AS builder
WORKDIR /app

# Copiamos la receta generada en la etapa anterior
COPY --from=planner /app/recipe.json recipe.json

# Construcción de dependencias (Caché masivo)
# Al estar en 'chef', ya estamos usando Nightly por defecto
RUN cargo chef cook --release --recipe-path recipe.json

# Inyección del código fuente completo
COPY . .

# Compilación del binario final del Orquestador
RUN cargo build --release --bin orchestrator -j 1

# --- ETAPA 4: RUNTIME (ESTRATO BLINDADO) ---
# Usamos una imagen ligera de Debian para el despliegue final
FROM debian:bookworm-slim AS runtime
WORKDIR /app

# 1. Instalación de herramientas de runtime
# CRÍTICO: 'curl' es obligatorio para que entrypoint.sh descargue los shards
RUN apt-get update && apt-get install -y \
    libssl3 \
    ca-certificates \
    curl \
    && rm -rf /var/lib/apt/lists/*

# 2. Extracción del binario desde la etapa de construcción
COPY --from=builder /app/target/release/orchestrator ./prospector-orchestrator

# 3. Inyección del script de arranque táctico
# Este script gestiona la descarga de los 4 fragmentos del censo
COPY scripts/entrypoint.sh ./entrypoint.sh
RUN chmod +x ./entrypoint.sh

# 4. Preparación de estructura de directorios
# Garantizamos que la ruta para los shards exista
RUN mkdir -p dist/filters/satoshi_era

# 5. Configuración operativa por defecto
ENV RUST_LOG=info
ENV PORT=3000

# Exposición del puerto para el balanceador de carga de Render
EXPOSE 3000

# 6. Ignición a través del script de control
ENTRYPOINT ["./entrypoint.sh"]
