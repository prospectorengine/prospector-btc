# 📘 CONCEPTO: INTERFAZ DE ALTO NIVEL PARA PUNTOS

**Módulo Objetivo:** `JacobianPoint` (L1)

## 1. El Problema de la Abstracción
Los motores de estrategia L2 operan con objetos `SafePrivateKey` para garantizar que los escalares cumplan con $0 < k < n$. El núcleo matemático L1, sin embargo, procesa ráfagas de bytes para maximizar el throughput.

## 2. La Solución: Puente de Identidad
Se ha inyectado el método `from_private` para:
-   **Compatibilidad Regresiva:** Permitir que los motores forenses sigan utilizando la lógica de objetos sin preocuparse por la manipulación de bytes.
-   **Seguridad de Tipos:** Garantizar que la ascensión al espacio Jacobiano siempre parta de una clave privada validada.

## 3. Optimización Transparente
Aunque la interfaz recibe un objeto, internamente dispara el motor de ventana fija de 4 bits, manteniendo la eficiencia de **Gold Master** sin alterar el código del consumidor.
