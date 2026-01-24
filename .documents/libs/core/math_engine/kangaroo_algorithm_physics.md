# 📘 CONCEPTO: KANGAROO SOLVER (POLLARD'S LAMBDA)

**Clasificación:** SOLUCIONADOR ECDLP (ESTRATO L1)
**Misión:** V22.0 - Resolución de Rango Corto

## 1. La Física del Problema
A diferencia del barrido secuencial ($O(N)$), el algoritmo Kangaroo está diseñado para encontrar un escalar $k$ si sabemos que reside en un intervalo $[A, B]$. Su complejidad es $O(\sqrt{B-A})$, lo que lo hace exponencialmente más rápido para búsquedas dirigidas.

## 2. Mecánica de Puntos Distinguidos (DP)
Para evitar el almacenamiento de cada paso (que agotaría la RAM de Colab), el sistema solo guarda "trampas" en puntos cuyas coordenadas cumplen con un predicado matemático (Máscara de bits).
- **Trayectoria Tame:** Salta desde el final del rango conocido y deja trampas.
- **Trayectoria Wild:** Salta desde el punto objetivo $Q$ buscando caer en una trampa.

## 3. Optimización Gold Master V22
- **Static Signatures:** El uso de `[u8; 33]` para las llaves del mapa elimina el overhead del puntero y la alocación dinámica, permitiendo que la CPU se dedique exclusivamente a la aritmética de la curva.
- **SipHash Routing:** Aunque el selector de salto es determinista, la distribución de la matriz asegura una cobertura estocástica del espacio de claves, previniendo ciclos infinitos.
