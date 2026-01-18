# 📘 CONCEPTO: PROJECTIVE SEQUENTIAL ENGINE

**Clasificación:** Aparato de Estrategia L2
**Estado:** V200.0 (Trinity Protocol)

## 1. Física del Aparato
Este motor es responsable de recorrer secuencialmente el espacio de claves privadas de Bitcoin ($2^{256}$) a máxima velocidad. A diferencia de un enfoque ingenuo que calcula cada clave pública de forma aislada, este motor utiliza la propiedad aditiva de la Curva Elíptica:
$$P_{n+1} = P_n + G$$
Donde $G$ es el punto generador. Esto es computacionalmente mucho más barato que $G \times (n+1)$.

## 2. Topología y Relaciones
- **Input:** Recibe un escalar inicial (Hex), un límite de iteraciones y el Filtro de Bloom (L1).
- **Dependencias:**
  - `libs/core/math-engine`: Provee la aritmética Jacobiana (`add_mixed_deterministic`) y de Campos (`batch_invert_sovereign`).
  - `libs/core/probabilistic`: Provee el `ShardedFilter` para verificación O(1).
- **Output:** Reporta colisiones vía `FindingHandler` y retorna un checkpoint hexadecimal.

## 3. Matemática: Coordenadas Jacobianas y Truco de Montgomery

### El Problema de la Inversión
En coordenadas afines $(x, y)$, sumar dos puntos requiere una división modular ($1/dx \pmod p$). La división es extremadamente lenta (100x más que una multiplicación).

### La Solución Jacobiana
Usamos coordenadas $(X, Y, Z)$ donde $x = X/Z^2$ y $y = Y/Z^3$. Esto permite sumar puntos sin dividir. Sin embargo, para verificar si la clave pública resultante está en el filtro (que guarda hashes de coordenadas afines), eventualmente *debemos* volver a afines (dividir por $Z^2$).

### Optimización de Montgomery (Batch Inversion)
Para amortizar el costo de la inversión, no convertimos cada punto inmediatamente.
1.  **Acumulación:** Guardamos 1024 puntos en un "Cargador" (Magazine).
2.  **Inversión Masiva:** Calculamos $I = (Z_1 \cdot Z_2 \dots Z_n)^{-1}$.
3.  **Despliegue:** Usamos $I$ para derivar $1/Z_i$ para cada punto individual usando solo multiplicaciones.

$$ CostoTotal = 1 Inversión + 3N Multiplicaciones $$
En lugar de $N$ Inversiones. Esto resulta en una aceleración dramática.

## 4. Justificación de Diseño
- **Batch Size 1024:** Elegido para caber en la caché L2 de la mayoría de CPUs modernas (x86_64), minimizando los fallos de caché durante el bucle caliente.
- **Zero-Allocation:** El bucle principal no realiza asignaciones en el Heap (`Vec::push` solo ocurre hasta llenar la capacidad pre-reservada).
- **Determinismo:** El uso de `add_mixed_deterministic` garantiza que la ejecución sea reproducible bit a bit en cualquier arquitectura.

---
