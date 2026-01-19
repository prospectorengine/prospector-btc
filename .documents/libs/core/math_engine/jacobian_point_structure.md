# 📘 CONCEPTO: SINCRONIZACIÓN NOMINAL DE ESTRATOS

**Módulo Objetivo:** `JacobianPoint` -> `FieldElement`

## 1. La Física del Enlace
En el Protocolo Hydra-Zero, los "Aparatos" deben ser intercambiables y auditables. La transición de `be` (abreviatura) a `big_endian` (nombre nominal) en la capa de Campo (`FieldElement`) rompió los punteros lógicos en la capa de Punto (`JacobianPoint`).

## 2. El Remedio Táctico
Se ha aplicado una nivelación de **Frontera de API**. El `JacobianPoint` ahora consume explícitamente:
1. `from_big_endian_bytes`: Para hidratar el punto desde material binario.
2. `internal_words_to_big_endian_bytes`: Para exportar el punto hacia el censo UTXO.

## 3. Impacto en la Tesis
Esta nivelación garantiza que la **Cadena de Verdad** sea ininterrumpida. Si un auditor lee el código en L1, no encontrará acrónimos ambiguos, sino descriptores precisos del orden de bytes esperado por el protocolo Bitcoin.
