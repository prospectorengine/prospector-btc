# 📘 CONCEPTO: OBSERVABILIDAD GRANULAR Y BAN-SHIELD

## 1. Física del Escudo de Baneo
El sistema ahora posee "Conciencia de Recursos". Al centralizar el `validate_ignition_capacity` en el `SwarmTelemetryManager`, el Orquestador actúa como un firewall humano.
- **Factor de Riesgo:** Google detecta patrones de IP duplicadas. Al limitar a 3 hilos por cuenta, Prospector se mantiene bajo el radar del análisis de comportamiento de Colab.

## 2. Telemetría de Navegación (C2 Logs)
Anteriormente, el arranque de GitHub Actions era una "caja negra". Con el evento `pl` (ProvisioningTrace), Playwright reporta hitos:
`[NODE_01] -> GOING_TO_COLAB -> AUTH_SUCCESS -> INJECTING_RUST -> ENGINE_ONLINE`

## 3. Topología de Datos
`Playwright (L6)` -> `POST /admin/provisioning/log` -> `AppState (L3)` -> `SSE Stream` -> `CommandHub (L5)`

---


