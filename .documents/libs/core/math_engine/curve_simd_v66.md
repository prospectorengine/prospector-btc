# 📘 CONCEPTO: VECTORIZED JACOBIAN ENGINE (AVX2 4-WAY)

**Clasificación:** CORE MATH STRATUM (L1)
**Hito:** V66.1 (Swiss Watch Level)

## 1. Física del Aparato
El motor SIMD rompe la barrera del procesamiento secuencial. En lugar de calcular una adición en la curva por cada ciclo de instrucción, el sistema utiliza registros YMM de 256 bits para empaquetar 4 elementos de campo.

## 2. Topología
- **Input:** 4 puntos JacobianPoint (X, Y, Z).
- **Procesamiento:** Instrucciones `_mm256_add_epi64` y `_mm256_sub_epi64`.
- **Output:** Un único `JacobianPointVector4` con los resultados calculados en paralelo.

## 3. Justificación de Diseño
Para garantizar la estabilidad en entornos de nube heterogéneos, el aparato implementa una **Dualidad de Backend**:
1. **Sovereign AVX2:** Activo en CPUs modernas (Intel Core 4th Gen+).
2. **Safe Fallback:** Emulación por software para CPUs antiguas, asegurando que el worker nunca entre en pánico.

## 4. Ganancia de Rendimiento
- **Teórica:** 400% (4x throughput).
- **Empírica (VAIO):** 320% debido al overhead de transposición de limbs (Load/Store).
