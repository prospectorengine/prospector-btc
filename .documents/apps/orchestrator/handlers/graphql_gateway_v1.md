# 📘 CONCEPTO: GRAPHQL HTTP ADAPTER V1.0

## 1. Integración Axum-AsyncGraphQL
Este aparato utiliza `async-graphql-axum` para extraer automáticamente el contexto HTTP (Headers, Body) y pasarlo al motor de ejecución.
- **Eficiencia:** La deserialización del JSON y la ejecución del grafo ocurren en el mismo paso asíncrono, minimizando copias de memoria.

## 2. Playground (Academia)
El endpoint `/api/v1/graphql/playground` sirve una SPA (Single Page Application) ligera embebida en el binario. Esto permite a los estudiantes de la Academia inspeccionar la documentación del esquema (tipos, queries) sin salir del navegador, cumpliendo el objetivo de "Auto-Descubrimiento".
