# 📘 CONCEPTO: BILLING ATOMIC STRATA (V1.1)

**Clasificación:** INFRASTRUCTURE RESILIENCE (ESTRATO L3)
**Hito:** V1.1 - Sincronía Zenith

## 1. El Combustible del Enjambre
Cada misión de minería Jacobiana consume recursos finitos. Para cuantificar este esfuerzo, el sistema utiliza **Créditos de Energía**. La integridad de estos créditos es vital para el modelo de negocio SaaS.

## 2. La Transacción Galvánica
El repositorio V1.1 implementa una transacción dual:
- **Write-Ahead (L3):** Se descuenta el balance del caché local para permitir una respuesta < 10ms al worker.
- **Outbox-Log (L4):** Se sella la intención de gasto para que el Relay la transmita al Motor B.

## 3. Justificación de value_text
SQLite almacena números de punto flotante, pero la conversión entre lenguajes (Rust <-> SQL <-> TS) puede introducir errores de redondeo. Almacenar el balance como `TEXT` y parsear en el Hot-Path de L3 garantiza que la precisión de los 256 bits se mantenga intacta a nivel informativo.
