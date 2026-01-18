# 📘 CONCEPTO: NEURAL ORACLE CLIENT V1.0

## 1. Abstracción Unificada
El cliente `neuralOracle` permite al Dashboard realizar consultas sin conocer la complejidad del transporte HTTP.
- **Antes:** `axios.post('/api/v1/graphql', { query: ... })` (Verbosidad).
- **Ahora:** `neuralOracle.query('{ status }')` (Semántica).

## 2. Manejo de Errores de Grafo
A diferencia de REST, GraphQL siempre retorna `200 OK` incluso si hay errores lógicos. El cliente inspecciona el campo `errors` del payload JSON y, si existe, rechaza la promesa, permitiendo que `React Query` maneje el estado `isError` correctamente en la UI.
