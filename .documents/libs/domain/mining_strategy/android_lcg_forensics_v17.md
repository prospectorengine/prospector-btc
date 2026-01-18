Ubicación: .documents/libs/domain/mining_strategy/android_lcg_forensics_v17.md
Física del Aparato: Este motor explota la vulnerabilidad histórica de Android donde el PRNG de Java (java.util.Random) era inicializado con semillas de solo 48 bits (basadas en el tiempo del sistema). Esto reduce el espacio de búsqueda de
2
256
2
256

 a
2
48
2
48

, un rango que el enjambre Hydra-Zero puede auditar exhaustivamente.
Mecánica de Barrido: El test certifica que el AndroidLcgIterator replica bit-a-bit la secuencia de salida de la JVM. Valida que al consumir 32 bytes de entropía del iterador, se produce la misma clave privada que generaría un teléfono Android vulnerable en 2013.
Topología Neural: El test se conecta al Orquestador L3 para inyectar su reporte. Si el rendimiento cae por debajo de un umbral, el Dashboard alertará al operador sobre una posible ineficiencia en la arquitectura de hilos de la instancia de Google Colab.
