# 📘 CONCEPTO: DETERMINISTIC PROBABILISTIC FILTER V24.0

## 1. El Problema de la Portabilidad Binaria
Los motores de serialización como `bincode` por defecto utilizan el "Native Endianness" de la máquina.
- Si el Censo se genera en un Servidor Intel (Little Endian) y se despliega en un dispositivo IoT (Big Endian), el filtro sería ilegible.
- Además, el uso de enteros de longitud variable (VarInt) ahorra espacio pero añade complejidad de CPU.

## 2. Solución Soberana
Hemos forzado la configuración del serializador a:
1.  **Little Endian:** Estándar de facto para x86_64 y ARM64 (Apple Silicon, AWS Graviton).
2.  **Fixed Integer:** `u64` siempre ocupa 8 bytes. Esto desperdicia unos bytes pero hace que la lectura de memoria sea predecible y alineada, vital para `mmap`.

## 3. Impacto en el Enjambre
Esta actualización permite que el artefacto `utxo_filter.bin` sea "agnóstico de arquitectura". Un nodo en Raspberry Pi (ARM) puede consumir el mismo archivo binario que un servidor Xeon de alto rendimiento, facilitando la expansión horizontal del enjambre.
