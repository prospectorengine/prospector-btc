# 📘 CONCEPTO: SOVEREIGN GALVANIC RELAY (OUTBOX PATTERN)

**Clasificación:** INFRASTRUCTURE RESILIENCE (ESTRATO L4)
**Hito:** V200.1 - Zenith Absolute Alignment

## 1. Física del Problema: El Abismo de Red
En una arquitectura distribuida, la comunicación directa entre el Orquestador y Supabase (Motor B) durante una ráfaga de minería es un vector de fallo. Si Supabase presenta latencia, el hilo de la API se bloquea, degradando el hashrate global.

## 2. La Solución: Sincronía Galvánica
Implementamos el **Patrón Outbox**. La mutación de datos (Billing, XP, Certificaciones) se sella primero en el Ledger Táctico (Turso - Motor A) dentro de la misma transacción atómica de la misión.

El `SovereignRelayService` actúa como un "marcapasos de datos":
1. **Polling O(1):** Escanea la tabla `outbox_strategic` buscando registros en estado `pending`.
2. **Idempotencia 409:** Al transmitir a Supabase, si se recibe un conflicto (409), el relay entiende que la verdad ya reside en el HQ y procede al sellado local.
3. **Backoff Exponencial:** Ante fallos 5xx, el servicio incrementa el tiempo de espera para proteger el ancho de banda del túnel neural.

## 3. Topología de Datos
`Transacción L3` -> `Motor A (Outbox)` -> `Relay Daemon (L4)` -> `Motor B (Strategic HQ)`
