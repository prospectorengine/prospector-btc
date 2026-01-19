# 📘 CONCEPTO: ALGORITMO KANGAROO (POLLARD'S LAMBDA)

**Módulo Objetivo:** `KangarooSolver` (L1)

## 1. Física del Problema
A diferencia del barrido secuencial, que es $O(n)$, el algoritmo de los Canguros de Pollard busca resolver el Problema del Logaritmo Discreto (ECDLP) en un tiempo $O(\sqrt{w})$, donde $w$ es el ancho del rango. Es ideal para misiones donde se conoce que una clave pertenece a un segmento específico de la curva.

## 2. Sincronía Galvánica (V19.1)
El motor ha sido nivelado para utilizar la aritmética nominal `big_endian`. Esto asegura que los saltos en la curva ($P + step$) se calculen con precisión de 256 bits sin errores de acarreo o de nomenclatura.

## 3. Estrategia de Puntos Distinguidos (DP)
Para optimizar la memoria en el Orquestador, no guardamos todos los saltos del canguro "Tame", solo aquellos cuyas coordenadas cumplen con una máscara de bits (`distinguished_point_mask`). Esto permite que el enjambre distribuido trabaje con trampas de solo unos pocos megabytes en RAM.
