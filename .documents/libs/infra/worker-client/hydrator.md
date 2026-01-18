# 📘 CONCEPTO: FORENSIC DNA HYDRATOR

**Clasificación:** Infraestructura L3
**Estado:** V45.3 (Reparado)

## 1. Física del Aparato
Este componente es el encargado de **materializar** los activos binarios necesarios para la simulación forense (Satoshi-XP) en la memoria RAM del trabajador. Específicamente, carga el archivo `WIN_XP_SP3.bin` (250KB) que contiene el snapshot del registro de Windows.

## 2. El Problema de Visibilidad (E0432)
En Rust, la jerarquía de módulos es privada por defecto. Aunque `hydrator.rs` existía físicamente en la carpeta `src`, no estaba declarado en `lib.rs` como `pub mod`. Esto lo hacía invisible para consumidores externos como `apps/miner-worker`.
La reparación consistió en exponer explícitamente el módulo y re-exportar el struct `ForensicDnaHydrator` en la raíz de la librería.

## 3. Flujo de Datos
1.  **Provisioner:** Inyecta la URL del binario.
2.  **Miner Kernel:** Detecta una misión tipo `SatoshiWindowsXpForensic`.
3.  **Hydrator:** Verifica si el archivo `.bin` está en disco. Si no, lo descarga del Orquestador (`/assets/dna/...`).
4.  **Validación:** Chequea la firma "PERF" en los primeros 4 bytes.
5.  **Inyección:** Entrega el `Vec<u8>` al `SatoshiWindowsXpForensicEngine`.

---


