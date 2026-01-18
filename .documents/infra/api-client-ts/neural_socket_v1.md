# 📘 CONCEPTO: NEURAL SOCKET CLIENT V1.0

## 1. Singularidad de Enlace
A diferencia de SSE, el cliente WebSocket mantiene una conexión TCP persistente y bidireccional.
- **Upstream:** Permite enviar comandos `{ cmd: "PAUSE" }` sin peticiones HTTP extra.
- **Downstream:** Recibe telemetría binaria comprimida (Base64).

## 2. Estrategia de Autenticación (Limitación de Navegador)
La API `WebSocket` del navegador no permite cabeceras personalizadas (`Authorization`).
**Solución Táctica:** El token se envía en el parámetro `protocols` (`sec-websocket-protocol`). El servidor debe estar configurado para inspeccionar este campo o el cliente debe migrar a Query Params (`?token=xyz`) en la Fase 3.
