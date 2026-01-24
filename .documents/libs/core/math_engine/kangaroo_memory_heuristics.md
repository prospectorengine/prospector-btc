# 📘 CONCEPTO: KANGAROO MEMORY HEURISTICS

**Clasificación:** GOBERNANZA DE RECURSOS (ESTRATO L1)
**Misión:** V23.0 - Prevención de Colapso por OOM (Out Of Memory)

## 1. El Problema del Almacenamiento Estático
El algoritmo de Pollard's Kangaroo requiere almacenar puntos "distinguidos" de una trayectoria maestra (Tame) para que las trayectorias de búsqueda (Wild) puedan colisionar con ellas.
Un límite estático (ej: 25,000 trampas) es:
- **Ineficiente** en hardware potente (Google Colab con 12GB).
- **Peligroso** en hardware limitado (contenedores de 512MB), causando pánicos de memoria (SIGKILL).

## 2. La Solución: Presupuesto Dinámico (Memory Sovereignty)
El motor ahora acepta un presupuesto en MB. La capacidad real de la `HashMap` se calcula mediante:
$$ C = \frac{Budget\_Bytes}{Estimated\_Footprint} $$

Hemos fijado el `ESTIMATED_TRAP_FOOTPRINT_BYTES` en **128 bytes**. Esto cubre:
- La llave de 33 bytes.
- El valor de 32 bytes.
- El puntero de entrada de la HashMap (~24-32 bytes).
- Un margen de seguridad para evitar re-alocaciones elásticas que fragmenten el heap.

## 3. Impacto en el Hashrate
Esta nivelación permite al worker saturar la RAM disponible para maximizar la probabilidad de éxito de la colisión sin supervisión manual, convirtiendo al nodo en un organismo autoconsciente de su entorno físico.
