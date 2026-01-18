# 📘 CONCEPTO: MOTOR MODULAR ESCALAR

**Módulo Objetivo:** `libs/core/math-engine/src/scalar.rs`

## 1. Física del Aparato
El `Scalar` representa una clave privada en el grupo cíclico de secp256k1. A diferencia de las coordenadas de los puntos, los escalares deben ser estrictamente menores que el orden $n$. Este aparato implementa la sustracción condicional para asegurar que cualquier número de 256 bits se mapee a un escalar válido.

## 2. Justificación de Seguridad
- **Exclusión de Cero:** Un escalar de 0 produciría el "Punto al Infinito", rompiendo la lógica de derivación de direcciones. El test garantiza que el sistema lance un error de tipo `InvalidKeyFormat`.
- **Determinismo de Reducción:** Se verifica que $n + x \equiv x \pmod n$ para asegurar que el escaneo secuencial no se salga de los límites de la curva.

## 3. Análisis de Rendimiento
La validación de escalares ocurre en cada iteración del minero. Este test mide la latencia de la reducción para asegurar que el "Handshake" escalar no sea un cuello de botella.
