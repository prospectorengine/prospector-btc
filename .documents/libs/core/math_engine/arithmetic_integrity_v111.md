Ubicación: .documents/libs/core/math_engine/arithmetic_integrity_v111.md
Física del Aparato: Este test certifica la base bit-a-bit del proyecto. Al operar con claves de 256 bits, el sistema debe gestionar 4 palabras de 64 bits simultáneamente. Validamos que la lógica de acarreo (Carry) fluya correctamente del registro 0 al 3 sin pérdida de datos.
Mecánica ASM: Se verifica que las instrucciones add y adc (Add with Carry) inyectadas mediante ensamblador inline en arithmetic.rs funcionen de forma determinista en el hardware objetivo.
Trazabilidad IA: El reporte emitido contiene la arquitectura (arch) y el sistema operativo (os). Esto permite que el Dashboard identifique si un fallo aritmético es específico de una instancia de Google Colab o un error algorítmico global.

---

