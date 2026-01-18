# 📜 PROTOCOLO TRINIDAD: DESARROLLO HOLÍSTICO BASADO EN EVIDENCIA
**Estado:** ACTIVO
**Mandato:** Cero Ambigüedad, Cobertura Total.

## 1. LA TRÍADA DE LA INGENIERÍA
Cada intervención en el código fuente (Refactorización o Creación) debe generar obligatoriamente tres artefactos sincronizados:

### A. EL APARATO (Source)
- **Ubicación:** `libs/...` o `apps/...`
- **Estándar:** Código limpio, tipado estricto, optimizado (SIMD/Async), sin abreviaciones.
- **Documentación Inline:** RustDoc/TSDoc exhaustivo en cada función pública.

### B. LA PRUEBA ESPEJO (Evidence)
- **Ubicación:** `tests/mirror/[ruta_del_aparato].test.rs` (o `.ts`)
- **Objetivo:** Replicar la ruta del archivo original dentro de la carpeta `tests/mirror/`.
- **Cobertura:** Prueba unitaria y de integración de **cada** funcionalidad expuesta.
- **Rigor:** Debe fallar si la lógica cambia (Regression Testing).

### C. EL CONCEPTO (Knowledge)
- **Ubicación:** `.documents/[ruta_del_aparato].md`
- **Contenido:**
    1.  **Física del Aparato:** ¿Qué problema resuelve y cómo? (Sin código, solo lógica).
    2.  **Topología:** Relación con otros aparatos (Inputs/Outputs).
    3.  **Matemática/Algoritmia:** Explicación teórica (ej: Montgomery Inversion, AES-GCM).
    4.  **Justificación de Diseño:** Por qué se tomaron ciertas decisiones técnicas.

## 2. FLUJO DE TRABAJO
1.  **Analizar:** Entender el aparato y sus dependencias.
2.  **Refactorizar:** Escribir el código fuente (Artefacto A).
3.  **Documentar:** Escribir el concepto (Artefacto C).
4.  **Probar:** Escribir y validar el test (Artefacto B).
5.  **Verificar:** Asegurar que A, B y C son coherentes entre sí.

---
# 📜 PROTOCOLO TRINIDAD: DESARROLLO HOLÍSTICO BASADO EN EVIDENCIA
**Estado:** ACTIVO (Rev. 1.1)
**Mandato:** Cero Ambigüedad, Cobertura Total, Ejecución Inmediata.

## 1. LA TRÍADA DE LA INGENIERÍA
Cada intervención en el código fuente debe generar tres artefactos sincronizados:

### A. EL APARATO (Source)
- **Ubicación:** `libs/...` o `apps/...`
- **Estándar:** Código limpio, tipado estricto, optimizado (SIMD/Async).
- **Documentación Inline:** RustDoc/TSDoc exhaustivo.

### B. LA PRUEBA ESPEJO (Evidence)
- **Ubicación:** `tests/mirror/[ruta_del_aparato].test.rs`
- **Rigor:** Replicación exacta de la ruta. Debe probar casos de borde y éxito.
- **Enlace:** Debe estar registrada en el `Cargo.toml` del aparato correspondiente bajo `[[test]]`.

### C. EL CONCEPTO (Knowledge)
- **Ubicación:** `.documents/[ruta_del_aparato].md`
- **Contenido:** Física, Topología y Justificación Matemática.

---

## 🚨 ADENDA TÁCTICA: PROTOCOLO DE DISPARO (WIN-10)
**"Un arma sin gatillo es inútil."**

Junto con la entrega del **Artefacto B (Prueba Espejo)**, es MANDATORIO entregar el **Comando de Disparo** validado para la terminal de Windows 10.

**Formato Estándar de Entrega:**
Debe incluirse al final de la respuesta de la IA en un bloque de código claramente etiquetado.

```bash
# COMANDO DE DISPARO (WIN-10 POWERSHELL/CMD)
cargo test --package [NOMBRE_PAQUETE] --test [NOMBRE_TEST_TOML] -- --nocapture

---

