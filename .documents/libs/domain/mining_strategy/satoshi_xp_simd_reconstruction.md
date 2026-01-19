# 📘 CONCEPTO: ARQUEOLOGÍA SIMD SATOSHI-XP

**Módulo Objetivo:** `SatoshiWindowsXpForensicEngine` (L2)

## 1. Física del Motor
El motor replica el comportamiento del cliente Bitcoin v0.1.x sobre Windows XP. Su mayor cuello de botella es la derivación de la clave pública tras agitar el pool de entropía.

## 2. Aceleración Vectorial (Fase Zenith)
Al sincronizar con el motor `JacobianPointVector4` (L1), el motor forense ahora procesa 4 trayectorias de tiempo (QPC ticks) simultáneamente.
- **Antes:** 1 tick por ciclo de instrucción.
- **Ahora:** 4 ticks inyectados en registros YMM de 256 bits.

## 3. Resolución Nominal
Se han eliminado las referencias a `x_strata_vector` y similares, adoptando los campos normalizados `x`, `y`, `z`. Esto garantiza que el compilador de Rust genere código ensamblador más limpio, reduciendo los "stalls" en el pipeline de la CPU del VAIO.
