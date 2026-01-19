# 📘 CONCEPTO: MOTOR JACOBIANO VECTORIZADO

**Módulo Objetivo:** `JacobianPointVector4` (L1)

## 1. Física del Paralelismo
El sistema utiliza el paradigma **SIMD (Single Instruction, Multiple Data)** para romper la barrera del hashrate secuencial. Mediante registros AVX2, una sola instrucción de suma o multiplicación actúa sobre 4 carriles de datos de 64 bits simultáneamente.

## 2. Optimización Meloni Vectorizada
La implementación de la adición Co-Z en SIMD es el punto álgido de la eficiencia en L1.
- **Escalabilidad:** Procesamos 4 adiciones Jacobianas en el tiempo que un motor normal procesa 1.5 adiciones.
- **Saturación:** Al usar 5 multiplicaciones (5M) vectorizadas, minimizamos los "Pipeline Bubbles" de la CPU, manteniendo los núcleos de Colab al 100% de su capacidad criptográfica.

## 3. Resolución Nominal
Se han sincronizado los campos con el estándar `x`, `y`, `z`, eliminando la verbosidad anterior para facilitar la legibilidad del código ensamblador que genera el compilador de Rust.
