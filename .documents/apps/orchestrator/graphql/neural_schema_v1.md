# 📘 CONCEPTO: NEURAL SCHEMA ARCHITECTURE V1.0

## 1. Topología del Grafo
El sistema GraphQL no reemplaza a la API REST, la complementa.
- **REST:** Alta frecuencia, latencia mínima (Workers).
- **GraphQL:** Consultas relacionales profundas (Dashboard Académico).

## 2. El Patrón QueryRoot
`QueryRoot` actúa como el despachador maestro. No contiene lógica de negocio compleja; su función es delegar a Sub-Resolvers (ej: `SystemResolver`, `MissionResolver`).
En esta Fase 1, implementamos `neural_gateway_status` para certificar que el mecanismo de Inyección de Dependencias (`ctx.data::<TursoClient>()`) funciona correctamente, evitando pánicos en runtime por contextos vacíos.
