# 📘 CONCEPTO: UNIFIED SCHEMAS V80 (NEURAL CORE)

## 1. Identificación de Regresiones Evitadas
Se han mantenido todas las estructuras de la V79.0 (Snapshot). La reducción de código previa fue un error de empaquetamiento. Esta versión garantiza que `NodeHardwareMetrics` y `WorkerHeartbeat` sigan siendo el estándar para el Orquestador Rust y el Dashboard Next.js.

## 2. Física de los Nuevos Eventos
- **`ProvisioningLog` (pl):** Permite al Dashboard visualizar el log de Playwright en GitHub Actions. Esto elimina la "caja negra" durante el arranque de los nodos.
- **`BanShieldStatus` (bs):** Provee un semáforo proactivo. Si el usuario intenta subir el número de hilos por encima de `identities * 3`, el sistema emite este evento para bloquear el botón de ignición en la UI.

## 3. Topología de Datos
Los eventos `pl` y `bs` viajan por el mismo túnel SSE que las métricas de hashrate, asegurando una sincronía temporal perfecta en el Dashboard.

---
