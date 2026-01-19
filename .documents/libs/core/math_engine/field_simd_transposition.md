# 📘 CONCEPTO: MOTOR DE CAMPO SIMD HÍBRIDO

**Módulo Objetivo:** `FieldElementVector4` (L1)

## 1. El Problema de la Big-Int Arithmética en SIMD
Las instrucciones SIMD nativas (como las de 256 bits de AVX2) están diseñadas para operar sobre múltiples números pequeños en paralelo. Nuestra necesidad es operar sobre 4 números gigantes (256 bits) simultáneamente.

## 2. La Solución: Transposición de Limbs
El aparato implementa una arquitectura donde:
- **Carril (Lane) i:** Contiene un elemento completo de 256 bits.
- **Registro YMM j:** Contiene la palabra `j` (64 bits) de los 4 carriles.

Esta disposición permite que una sola instrucción de suma (`VPADDQ`) procese la adición de 4 palabras de 64 bits en un solo ciclo, propagando los acarreos escalarmente durante la extracción o mediante lógica de carril cruzado.

## 3. Resolución de Documentación (Severidad 8)
Se ha nivelado el bloque `fallback_backend` para cumplir con la directiva `#![deny(missing_docs)]`. Esto asegura que incluso el código de respaldo para CPUs antiguas sea auditable y cumpla con los estándares de la Tesis MIT.
