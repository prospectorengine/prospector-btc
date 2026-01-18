#!/bin/bash
# =================================================================
# APARATO: TACTICAL ENTRYPOINT (V20.0 - DYNAMIC SHARDING)
# CLASIFICACIÓN: OPS INFRASTRUCTURE
# RESPONSABILIDAD: HIDRATACIÓN DE ESTRATOS BINARIOS E IGNICIÓN
#
# VISION HIPER-HOLÍSTICA:
# Sincronizado con el Kernel de Rust V20.0 (Bootstrap.rs).
# Garantiza que el número de fragmentos descargados coincida con
# la cuota definida en FILTER_SHARDS para evitar el modo mantenimiento.
# =================================================================

set -e

echo " "
echo " 💠 [IGNITION]: Hydra-Zero Orchestrator V20.0 (Sovereign Core)"
echo " ------------------------------------------------------------ "

# 1. AUDITORÍA DE ESTRATOS DE PERSISTENCIA (MOTOR A)
if [ -z "$DATABASE_URL" ]; then
    echo "❌ FATAL: DATABASE_URL is not defined. Tactical link impossible.";
    exit 1;
fi

if [ -z "$TURSO_AUTH_TOKEN" ]; then
    echo "❌ FATAL: TURSO_AUTH_TOKEN missing. Authority rejected.";
    exit 1;
fi

# 2. PREPARACIÓN DE DIRECTORIOS TÁCTICOS
# Alineado con la constante Bootstrap::SHARDS_DIRECTORY en Rust
TARGET_DIR="dist/filters/satoshi_era"
mkdir -p "$TARGET_DIR"

# 3. PROTOCOLO DE HIDRATACIÓN (UPLINK GITHUB RELEASES)
if [ -z "$FILTER_BASE_URL" ]; then
    echo "⚠️  [WARNING]: FILTER_BASE_URL not set. Orchestrator will start in DRY MODE."
else
    echo "📥 [HYDRATION]: Syncing Stratum Manifest & Shards from remote forge..."

    # Descarga del Manifiesto de Integridad (Sovereign SSoT)
    curl -L -f -s -o "$TARGET_DIR/stratum_manifest.json" "$FILTER_BASE_URL/stratum_manifest.json" || {
        echo "❌ [ERROR]: Failed to download stratum_manifest.json. Integrity check will fail.";
    }

    # CÁLCULO DINÁMICO DE FRAGMENTOS
    # Si FILTER_SHARDS no está definida, el enjambre asume 4 por defecto (Legacy Support)
    SHARDS_TO_FETCH=${FILTER_SHARDS:-4}
    ITER_LIMIT=$((SHARDS_TO_FETCH - 1))

    echo "   📦 Sharding Logic: Expected $SHARDS_TO_FETCH fragments."

    for i in $(seq 0 $ITER_LIMIT); do
        SHARD_FILE="filter_shard_$i.bin"
        if [ ! -f "$TARGET_DIR/$SHARD_FILE" ]; then
            echo "   ⬇️  Downloading Shard $i/$ITER_LIMIT..."
            # Descarga silenciosa con reintento ante fallos de red efímeros
            curl -L -f -s --retry 3 -o "$TARGET_DIR/$SHARD_FILE" "$FILTER_BASE_URL/$SHARD_FILE"
        else
            echo "   ✅ Shard $i cached and level."
        fi
    done

    echo "✨ [HYDRATION_COMPLETE]: Strata level synchronized with forge."
fi

# 4. TRANSFERENCIA DE CONTROL AL KERNEL SOBERANO
echo "🚀 [KERNEL]: Dispatched. Handing over to prospector-orchestrator binary..."
echo " "

exec ./prospector-orchestrator
