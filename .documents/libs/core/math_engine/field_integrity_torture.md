# 📘 CONCEPTO: CÁMARA DE TORTURA DE CAMPO FINITO

**Clasificación:** VERIFICACIÓN FORMAL POR ISOMORFISMO
**Módulo Objetivo:** `libs/core/math-engine/src/field.rs`

## 1. Física del Aparato
El motor de campo opera sobre el primo de secp256k1 utilizando reducción de Solinas (plegado de 512 bits). Debido a la complejidad de los acarreos (carry-propagation) en registros de 64 bits, las pruebas unitarias simples son insuficientes para detectar errores sutiles en los bordes de $2^{256}$.

## 2. Justificación Matemática
Utilizamos `num-bigint` como **Oráculo de Verdad**. Al ser una librería de precisión arbitraria, su lógica es el estándar contra el cual medimos nuestra implementación optimizada en ensamblador/intrínsecos.

## 3. Topología de Prueba
1. **Generación:** `proptest` genera 50,000 vectores aleatorios de 32 bytes.
2. **Transformación:** Se convierten los bytes a `limbs` (u64x4) para Prospector y a `BigUint` para el Oráculo.
3. **Ejecución:** Se disparan las operaciones `Add`, `Sub`, `Mul` e `Inv`.
4. **Sentencia:** Cualquier discrepancia bit-a-bit resulta en un pánico inmediato con volcado forense.

---
Ubicación: .documents/libs/core/math_engine/field_integrity_v18.md
Física del Aparato: Este test certifica la base atómica del sistema. Utiliza Inferencia de Oráculo comparando el motor Prospector (optimizado para CPU) contra num-bigint (precisión infinita).
Mecánica de Montgomery: Valida que la inversión por lotes produce resultados idénticos a
a
p
−
2
(
m
o
d
p
)
a
p−2
 (modp)
, garantizando que el SequentialEngine no genere claves falsas.
Topología Panóptica: Al finalizar, el test se conecta como un cliente al Orquestador, inyectando su veredicto en la consola de diagnósticos del Dashboard.

---


