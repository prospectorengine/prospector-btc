# 📘 CONCEPTO: SCALAR MODULAR ENGINE (ORDEN N)

**Clasificación:** ARITMÉTICA DE GRUPO (ESTRATO L1)
**Misión:** V13.2 - Integridad del Grupo Cíclico

## 1. El Espacio Escalar
En secp256k1, el grupo de puntos tiene un tamaño $n$. Las claves privadas operan en este espacio. Cualquier operación que resulte en un valor $\ge n$ debe ser reducida para permanecer dentro de los límites de la curva.

## 2. Aritmética de Bajo Nivel (ASM)
- **Acarreo Proyectivo:** La sustracción de $n$ se ejecuta en registros de 64 bits utilizando la instrucción `sbb` (Subtract with Borrow) en cadena, permitiendo que la CPU procese el número de 256 bits en 4 ciclos efectivos.
- **Atomicidad:** Al asegurar que $k \pmod n$ se aplique en el constructor, garantizamos que el resto de los estratos (L2, L3) operen siempre con "Material Legal".

## 3. Topología de Integridad
Este aparato es el cimiento de:
- **SequentialEngine:** Para avanzar $k \to k+1$.
- **KangarooSolver:** Para calcular el delta $k_{tame} - k_{wild} \pmod n$.
