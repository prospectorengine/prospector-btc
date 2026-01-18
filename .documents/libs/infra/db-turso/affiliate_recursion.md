📘 CONCEPTO: AFFILIATE RECURSIVE ENGINE

**Módulo Objetivo:** `AffiliateRepository`
**Problema Resuelto:** Fragmentación de la visibilidad del hashrate en redes multinivel.

## 1. Física de la Red
En el protocolo Hydra-Zero, los afiliados no son solo referidos, son **Proveedores de Cómputo Delegado**. El sistema debe ser capaz de incentivar la expansión del enjambre premiando al nodo raíz por la potencia total de su rama.

## 2. Topología de Datos
- **Input:** ID de afiliado raíz.
- **Procesamiento:** El motor ejecuta una **Recursión de Grafo** en el servidor de base de datos. En lugar de traer todos los registros a la memoria de Rust (O(N)), delega la suma al motor libSQL, reduciendo el tráfico de red y la presión sobre el Garbage Collector.

## 3. Justificación de Diseño
Se utiliza una **Recursive CTE** por su alta eficiencia en estructuras de árbol. Esto permite que el sistema soporte profundidades de red de hasta 100 niveles (límite de seguridad de SQLite) sin degradación de latencia para el Dashboard Zenith.
