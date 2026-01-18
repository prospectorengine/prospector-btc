# 📘 CONCEPTO: NEURAL GATEWAY TOPOLOGY V201.0

## 1. Estratificación de Endpoints
El orquestador ahora expone una tríada de interfaces en `/api/v1`:

| Ruta | Protocolo | Propósito |
| :--- | :--- | :--- |
| `/swarm/*` | REST (JSON) | Alta frecuencia. Workers reportando progreso. |
| `/stream/metrics` | WebSocket | Tiempo real. Control C2 y Telemetría. |
| `/graphql` | GraphQL | Consultas complejas y relaciones profundas. |

## 2. Política de Seguridad Unificada
Todos los puntos de entrada bajo `/api/v1` están protegidos por el middleware `auth_guard`. Esto simplifica la superficie de ataque: "Si no tienes Token, no ves nada".
El Playground GraphQL también está protegido, lo que requiere que el desarrollador inyecte manualmente el header `Authorization` en la interfaz del Playground para introspeccionar el esquema.
