# 📘 CONCEPTO: SHARDED FILTER ARCHITECTURE V37

## 1. El Problema de la Contención
En un filtro de Bloom monolítico, la carga de 1GB de datos bloquea un solo hilo de ejecución. En infraestructuras como Render o Colab, esto desperdicia el paralelismo del hardware.

## 2. Solución: Fragmentación Determinista
La V37.0 utiliza **SipHash-1-3** para distribuir las direcciones Bitcoin entre N particiones. Esto permite:
- **Descargas en Paralelo:** El Worker descarga los 4 shards simultáneamente (4x velocidad de ignición).
- **Mmap Parcial:** El kernel de Linux solo mapea en RAM los shards que reciben consultas, optimizando el uso de memoria en nodos pequeños.

## 3. Integridad de Manifiesto
El Orquestador valida que el número de archivos `.bin` en disco coincida con `total_partition_count` antes de autorizar el modo OPERACIONAL, evitando falsos negativos por fragmentos perdidos.
