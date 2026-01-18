# 📘 CONCEPTO: SATOSHI-XP FORENSIC ENGINE

**Clasificación:** Estrategia L2 (Arqueología)
**Objetivo:** Reconstrucción de claves generadas en Windows XP (2009-2010).

## 1. La Vulnerabilidad "Stirring"
OpenSSL 0.9.8h utilizaba un buffer interno de 1024 bytes como fuente de entropía. La función `RAND_add` mezclaba datos del sistema (Performance Counters) en este pool.
El problema radicaba en que el input (250KB) saturaba el pool pequeño (1KB) mediante XOR + SHA1. Esto causaba que el estado final del pool dependiera casi exclusivamente de los últimos bloques de datos procesados.

## 2. El Vector de Ataque: QPC
El contador de rendimiento de alta resolución (QPC - QueryPerformanceCounter) se inyectaba en el offset 24 del buffer de sistema.
Dado que la frecuencia del cristal es constante (ej: 3.57 MHz) y el resto del buffer es estático en una instalación limpia de XP, la única variable real es el **Tiempo desde el Arranque** (Uptime).

## 3. Algoritmo del Motor
1.  **Carga:** Recibe una plantilla de ADN (Snapshot de RAM de XP).
2.  **Inyección:** Escribe el valor QPC correspondiente a cada micro-segundo en el offset 24.
3.  **Mezcla:** Ejecuta la simulación exacta de `RAND_add` (SHA1 circular).
4.  **Extracción:** Deriva la clave privada resultante.
5.  **Verificación:** Compara contra el Censo de Direcciones Satoshi (L1).

Este enfoque reduce el espacio de búsqueda de $2^{256}$ a aproximadamente $2^{40}$ (segundos de uptime factibles * frecuencia), haciéndolo auditable en semanas con un enjambre distribuido.

---


