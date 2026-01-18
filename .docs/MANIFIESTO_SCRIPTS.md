# 📜 MANIFIESTO DE EJECUCIÓN SOBERANA (HYDRA-ZERO)

## 1. Filosofía de Comandos
Cada script en el proyecto PROSPECTOR debe ser **Atómico**, **Observable** y **Resiliente**. No se permiten "scripts de una sola línea" sin gestión de errores.

## 2. Jerarquía de Estratos
- **L1 (Dev):** Enfocado en velocidad. Utiliza el caché de Nx para recompilación incremental.
- **L3 (Infra):** Handshakes obligatorios con Turso/Supabase antes de alterar el estado.
- **L4 (QA):** El "Protocolo Trinidad". Todo código nuevo debe pasar el `audit:logic` (Lint + Test).
- **L5 (Deploy):** Comandos de solo lectura o empaquetado final. `build:web` es la autoridad para Vercel.

## 3. Protocolo de Despliegue (Pre-Flight)
Antes de realizar un `git push` a `main`, el operador debe recibir "Semáforo Verde" en:
1. `pnpm preflight`: Valida tipos, traducciones y compilación de Rust.
2. `pnpm audit:system`: Valida que los endpoints remotos de producción estén vivos.

## 4. Convención de Nomenclatura
- `[estrato]:[acción]`: Comandos de propósito general (ej. `db:migrate`).
- `[estrato]:[acción]:[sub-acción]`: Comandos granulares (ej. `db:turso:pulse`).
