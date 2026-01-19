# 📘 CONCEPTO: FÍSICA DEL MOTOR MODULAR ESCALAR

**Módulo Objetivo:** `Scalar` (L1)

## 1. La Diferencia entre Campos y Grupos
En secp256k1, operamos sobre dos módulos distintos:
-   **Coordenadas (p):** Definen el campo finito donde viven los puntos.
-   **Escalares (n):** Definen el número total de puntos. Las claves privadas viven en este espacio.

## 2. Resolución de Severidad 8
La inyección de documentación en el struct `Scalar` y sus métodos cumple con el estándar de **Tesis Doctoral**, explicitando la base matemática de la reducción atómica. Se ha documentado específicamente el uso de ensamblador inline para la sustracción de precisión múltiple.

## 3. Nomenclatura Soberana
La migración de `from_u256_be` a `from_u256_big_endian` sella la brecha nominal en el núcleo matemático, garantizando que el rastro forense de la clave privada sea inequívoco durante la auditoría.
