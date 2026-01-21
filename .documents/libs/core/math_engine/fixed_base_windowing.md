# 📘 CONCEPTO: FIXED-BASE WINDOWING (TABLA CUÁNTICA)

**Clasificación:** OPTIMIZACIÓN GEOMÉTRICA (ESTRATO L1)
**Hito:** V1.0 - Sincronía Zenith

## 1. Física del Aparato
En la derivación estándar, computar $Q = k \cdot G$ requiere la técnica de "Double-and-Add", lo que implica 256 duplicaciones y ~128 adiciones Jacobianas. Al ser $G$ un punto fijo, podemos pre-computar sus múltiplos.

Utilizamos una **Ventana de 4 bits**. Esto divide el escalar de 256 bits en 64 "nibbles". Para cada posición de nibble, pre-calculamos los 15 posibles valores resultantes.

## 2. Topología
- **Input:** Escalar $k$ (32 bytes).
- **Procesamiento:** 64 consultas a `GENERATOR_TABLE` + 64 adiciones mixtas Jacobianas.
- **Output:** Punto Jacobiano $Q$.

## 3. Justificación de Diseño
- **Mixed Addition:** Al guardar los puntos en la tabla en formato Afín ($Z=1$), la suma Jacobiana se simplifica, ahorrando 3 multiplicaciones de campo por paso.
- **Cache Locality:** La tabla pesa ~61KB, lo que permite que resida casi enteramente en la caché L1/L2 del procesador, eliminando latencia de acceso a RAM.
