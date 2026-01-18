# 📘 CONCEPTO: NEURAL EVENT BUS V82.0

## 1. Topología de Difusión (Broadcast)
El sistema utiliza un canal de `broadcast` de Tokio. A diferencia de un canal `mpsc`, el broadcast permite que **múltiples consumidores** (WebSocket del Dashboard, Logger de Persistencia, Auditor de Salud) escuchen la misma señal simultáneamente.

## 2. Estrategia "Fail-Silent"
En sistemas de telemetría de alta frecuencia, el bus no debe bloquear la lógica de negocio. Si un emisor intenta enviar y no hay suscriptores (ej: Dashboard cerrado), el `EventBus` descarta la señal silenciosamente (`let _ = ...`), protegiendo el ciclo de CPU.

## 3. Resolución de Ceguera UI
Al añadir métodos explícitos para `emit_infrastructure_report` y `emit_visual_frame_signal`, garantizamos que cada cambio en el Orquestador sea visible para el operador, eliminando los estados `undefined` que causaban el crash en la interfaz.
