# 📘 CONCEPTO: SOBERANÍA DEL PRELUDIO MATEMÁTICO

**Módulo Objetivo:** `libs/core/math-engine/src/lib.rs`

## 1. El Problema del Acoplamiento
En versiones previas, los motores de búsqueda L2 dependían de nombres abreviados (`_be`) que oscurecían la física del dato en auditorías forenses.

## 2. La Solución: Preludio Normalizado
Se establece el `prelude` como la única interfaz de exportación autorizada para el núcleo matemático. Al normalizar los nombres a `big_endian`:
1.  **Auditoría Forense:** Se explicita la disposición de los bytes para comparaciones bit-a-bit con la red Bitcoin.
2.  **Cero Regresiones:** El compilador detecta inmediatamente cualquier discrepancia nominal en la cadena de mando.

## 3. Seguridad de Compilación
Al permitir `unsafe` a nivel de Hub, el sistema puede heredar las optimizaciones de ensamblador de `arithmetic.rs` y `scalar.rs` sin violar las restricciones de integridad de los sub-módulos, permitiendo el despliegue de binarios MUSL de alto rendimiento.
