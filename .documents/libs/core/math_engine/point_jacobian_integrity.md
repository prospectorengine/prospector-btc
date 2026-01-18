# 📘 CONCEPTO: MOTOR DE PUNTOS GEOMÉTRICOS

**Módulo Objetivo:** `libs/core/math-engine/src/point.rs`

## 1. Física del Aparato
En el espacio Jacobiano, un punto se representa como $(X, Y, Z)$. Para Bitcoin, necesitamos el punto Afín $(x, y)$. La relación es:
$x = X / Z^2 \pmod p$
$y = Y / Z^3 \pmod p$

Este aparato realiza la "Proyección de Retorno" necesaria para comparar hallazgos contra el filtro de Bloom.

## 2. Justificación de Diseño
- **Z-Inversion:** El test garantiza que el inverso modular de $Z$ se calcula correctamente utilizando el Pequeño Teorema de Fermat ($Z^{p-2} \pmod p$).
- **Soberanía Bit-Perfect:** Se verifica que al cargar un punto Afín con $Z=1$, la conversión de vuelta sea idéntica al input, validando que no hay errores de redondeo (imposibles en aritmética entera pero posibles por lógica de acarreo).

## 3. Impacto en Rendimiento
La proyección es la operación más lenta del ciclo de minería (debido al inverso). Este test mide cuántas proyecciones puede hacer un hilo para optimizar el tamaño de ráfaga (Magazine) en el motor secuencial.
