# 📘 CONCEPTO: GENERADOR DE DIRECCIONES LEGACY

**Módulo Objetivo:** `libs/core/generators/src/address_legacy.rs`

## 1. Física del Aparato
Este aparato implementa el estándar de Bitcoin para direcciones `P2PKH`. Realiza la transformación de una clave pública (Punto en curva) hacia un identificador de 20 bytes (Hash160) y finalmente aplica la codificación `Base58Check`.

## 2. Optimización de Élite
- **Zero-Allocation:** Se ha validado que el motor no utiliza el Heap para concatenar prefijos, operando directamente sobre buffers de stack ([u8; 65]).
- **Throughput:** El test mide el costo del doble hashing SHA256 y el RIPEMD160, garantizando que el worker pueda transformar colisiones Jacobianas sin latencia perceptible.

## 3. Justificación de Verdad
Se utiliza el vector "satoshi" (Block 1) como ancla de verdad. Si la dirección resultante cambia, significa que la lógica de Checksum o el orden de bytes del Hash160 se ha corrompido.
