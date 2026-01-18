# 📘 CONCEPTO: MOTOR SECUENCIAL PROYECTIVO

**Módulo Objetivo:** `libs/domain/mining-strategy/src/engines/sequential_engine.rs`

## 1. Física del Aparato
El motor opera bajo la premisa de que $P_{n+1} = P_n + G$. Al usar coordenadas Jacobianas, la adición se reduce a multiplicaciones de campo, eliminando la inversión modular (división) del bucle crítico.

## 2. Inversión por Lotes de Montgomery
Para el hashing Bitcoin (Hash160), se requieren coordenadas Afines. El motor acumula 1024 puntos ("Magazine") y realiza una **Inversión Simultánea** utilizando el algoritmo de Montgomery. Esto permite verificar 1024 llaves pagando el costo de una sola inversión modular, acelerando el proceso en un ~98%.

## 3. Estrategia de Certificación
- **Detección Dual:** Valida que el motor detecta colisiones tanto en formato Comprimido (0x02/03) como No-Comprimido (Satoshi 0x04).
- **Consistencia de Checkpoint:** Garantiza que el valor devuelto al Orquestador sea exactamente el escalar siguiente, permitiendo la reanudación atómica de misiones.
