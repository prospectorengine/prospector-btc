# 📘 CONCEPTO: SWARM TELEMETRY & VIRTUAL BIOMETRICS

## 1. Física de la Salud del Nodo
El orquestador no delega la seguridad física al worker. Al implementar `is_node_healthy`, el sistema centralizado decide si un nodo es apto para recibir una misión, protegiendo la longevidad de las VMs en la nube y evitando el desperdicio de rangos en nodos inestables.

## 2. El Ban-Shield (Capa L3)
Actúa como un **Límite de Densidad de Identidad**.
- **Lógica:** Si múltiples nodos acceden desde IPs similares con la misma cookie, Google dispara el Ban.
- **Remedio:** El ratio 1:3 garantiza que las cookies en la bóveda se distribuyan de forma que el comportamiento del enjambre parezca humano/orgánico.

## 3. Buffer de Navegación
Habilita el "Túnel de Visión". Los logs de Playwright se almacenan en RAM para ser consumidos por el Dashboard vía SSE, eliminando la latencia de disco en la telemetría de arranque.

---


