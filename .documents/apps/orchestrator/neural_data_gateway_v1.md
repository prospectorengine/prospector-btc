# 📘 CONCEPTO: NEURAL DATA GATEWAY (V1.0)

## 1. El Oráculo de Datos
El sistema evoluciona de una API REST rígida a un Grafo de Datos flexible.
- **REST (Táctico):** Se mantiene para operaciones de alta frecuencia (Workers, Heartbeats) donde la latencia es crítica.
- **GraphQL (Estratégico):** Se introduce para el Dashboard, permitiendo consultas complejas anidadas (ej: "Dame todos los nodos que minaron > 1GH/s en la última hora y sus hallazgos asociados").

## 2. Inyección de Contexto
El `Schema` de `async-graphql` actúa como un contenedor de dependencias. Al inyectar `database_client` y `event_bus` en el momento de la construcción (`state/mod.rs`), garantizamos que cualquier Resolver (Query/Mutation) pueda acceder a la infraestructura sin necesidad de pasar referencias globales, manteniendo la arquitectura limpia y testeable.
