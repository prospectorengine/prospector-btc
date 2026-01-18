# 📘 CONCEPTO: NEURAL COMMAND SOCKET (V1.0)

## 1. Transición SSE -> WS
El paso de Server-Sent Events a WebSockets elimina la restricción unidireccional.
- **Antes (SSE):** El servidor empujaba datos. El cliente debía abrir otra conexión HTTP (POST) para responder o comandar.
- **Ahora (WS):** Un solo tubo TCP persistente maneja tráfico en ambas direcciones.

## 2. Topología de Mensajes
### Downstream (Server -> Client)
Se mantiene el formato `BinaryNeuralPacker` (Base64 + MessagePack). Esto garantiza "Cero Regresiones" en el `NeuralCodec` del frontend, que espera decodificar strings Base64.
### Upstream (Client -> Server)
Canal JSON puro para comandos de control.
- `{"cmd": "PAUSE"}`
- `{"cmd": "SET_STRATEGY", "params": {...}}`

## 3. Manejo de Concurrencia
Utilizamos `tokio::spawn` para dividir el socket en dos tareas independientes (`send_task` y `recv_task`). El `tokio::select!` actúa como un fusible: si una dirección falla (ej: cliente cierra ventana), se termina todo el proceso para liberar memoria.
