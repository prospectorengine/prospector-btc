# 📘 CONCEPTO: TELEMETRY INGESTION GATEWAY

## 1. El Rol del Ingestor
Este aparato actúa como el punto de convergencia para logs heterogéneos. Permite que el automatizador de Playwright (Node.js) envíe sus errores de navegación al Orquestador (Rust), el cual los retransmite al Dashboard (React) en milisegundos.

## 2. El Ciclo del Pulso (Heartbeat)
El `spawn_telemetry_loop` es el marcapasos del sistema. Cada 5 segundos realiza un "congelamiento" (Snapshot) del estado de los workers en RAM para calcular las métricas agregadas. Este desacoplamiento protege a la base de datos Turso de miles de lecturas por segundo.

## 3. Resolución de Errores de Tesis
Al fijar `timestamp_ms` en el Orquestador, eliminamos la discrepancia horaria entre el reloj del servidor y el reloj del navegador del operador, permitiendo una medición real del RTT (Round Trip Time) de la red.
