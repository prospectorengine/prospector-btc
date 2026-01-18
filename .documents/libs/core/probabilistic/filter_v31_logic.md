# 📘 CONCEPTO: RICH LIST FILTER V31.0

## 1. El Problema de la Serialización de Bloom
Las estructuras probabilísticas son sensibles al orden de los bits. Si el Cartógrafo (L6) guarda el filtro en una arquitectura Big-Endian y el Worker (L1) lo lee en Little-Endian, el filtro retornará basura. La V31.0 fuerza `LittleEndian` a nivel de `bincode`.

## 2. Resolución de Trait Bounds
El fallo previo `the trait bound Bloom<[u8; 20]>: serde::Deserialize<'de> is not satisfied` se ha resuelto mediante la correcta configuración de la feature "serde" en la dependencia del workspace.

## 3. Optimización para Google Colab
La implementación de `load_from_file_mmap` es vital para el despliegue. Permite que el proceso de minería comience instantáneamente sin esperar a que 500MB de censo sean copiados físicamente a la RAM, delegando la carga bajo demanda al kernel de Linux.
