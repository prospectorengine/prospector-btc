# 📘 CONCEPTO: SCHEMA STITCHING V1.0

## 1. Fusión de Estratos
Hemos completado la fusión entre el Estado de Aplicación (`AppState`) y el Motor de Resolución (`QueryRoot`).
- **Antes:** `AppState` contenía un esquema vacío (`EmptyQuery`).
- **Ahora:** `AppState` contiene un esquema vivo capaz de interrogar al sistema.

## 2. Inyección de Dependencias
El patrón `Schema::build(...).data(...)` garantiza que cada vez que GraphQL resuelva un campo, tendrá acceso seguro (Thread-Safe) a:
1.  **Motor A (Turso):** Para consultas de datos.
2.  **EventBus:** Para suscripciones futuras (WebSockets).
