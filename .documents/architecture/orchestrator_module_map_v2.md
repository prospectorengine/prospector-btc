# 📘 CONCEPTO: ORCHESTRATOR MODULE TOPOLOGY V2.0

## 1. Expansión del Mapa
La inclusión de `graphql` en el `lib.rs` eleva el sistema de una arquitectura puramente REST a una Híbrida.
- **Antes:** Solo `handlers/` exponía lógica de API.
- **Ahora:** `graphql/` expone una interfaz tipada y autodescriptiva paralela.

## 2. Principio de Visibilidad
Mantenemos `pub mod` en la raíz para permitir que los tests de integración (`tests/mirror`) accedan a los componentes internos como si fueran consumidores externos, validando la API pública real de la librería Rust.
