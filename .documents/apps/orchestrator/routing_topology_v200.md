# 📘 CONCEPTO: SINGULARITY ROUTING MATRIX V200.0

## 1. El Túnel Neural (Upgrade)
El endpoint `/api/v1/stream/metrics` ha evolucionado de un emisor SSE unidireccional a un **Túnel WebSocket**.
- **Impacto:** Permite al Dashboard enviar comandos de "Pausa", "Cambio de Estrategia" o "Solicitud de Diagnóstico" sin abrir nuevas conexiones HTTP.
- **Latencia:** Reduce el overhead de handshakes TCP repetitivos.

## 2. Seguridad de Capa
El túnel permanece protegido por el `auth_guard`. Esto implica que el cliente (Dashboard) debe ser capaz de pasar el `Authorization: Bearer` durante el handshake, o en el futuro, migraremos a autenticación por Query Param (`?token=xyz`) si los navegadores presentan restricciones.
