# 📘 CONCEPTO: SOBERANÍA ARITMÉTICA U256

**Módulo Objetivo:** `arithmetic.rs` (L1)

## 1. El Problema de la Representación
Las claves de Bitcoin son números de 256 bits, pero las CPUs operan nativamente en 64 bits. El motor debe transformar continuamente entre:
1.  **Formato de Red (Big-Endian Bytes):** El estándar de Bitcoin para direcciones y WIF.
2.  **Formato de Cómputo (Little-Endian Limbs):** El estándar óptimo para registros de CPU.

## 2. Decisión Arquitectónica: Zero Abbreviations
Se ha prohibido el uso de `be` y `le`. El código ahora utiliza `big_endian` y `little_endian` para:
-   Eliminar ambigüedades en auditorías forenses.
-   Facilitar la comprensión de la Tesis por parte de agentes externos (humanos o IA).

## 3. Optimización ASM
El uso de `unsafe` y ensamblador inline es la única vía para acceder a la bandera de acarreo (Carry Flag) del procesador, reduciendo las sumas de 256 bits de una operación por byte a una operación por palabra de 64 bits.
