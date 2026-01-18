# 📘 CONCEPTO: KERNEL ARITMÉTICO SOBERANO

**Propósito:** Validar las operaciones de manipulación de bits de 256 bits utilizadas para el conteo de hashrate y navegación de rangos.

## 1. Física del Aparato
El motor utiliza `Big-Endian` para la representación de claves Bitcoin y `Little-Endian` para el procesamiento interno de la CPU. Este test certifica que el cambio de endianness (Limb conversion) no invierta el orden de significancia de los bytes.

## 2. Optimización ASM
En arquitecturas `x86_64`, se utiliza ensamblador inline (`add`, `adc`) para propagar el acarreo entre los 4 registros de 64 bits en un solo ciclo de CPU. El test de overflow garantiza que el flag de acarreo (Carry Flag) sea capturado correctamente por Rust.

## 3. Estratégia de Certificación
Se bombardea el método con 5 millones de incrementos para asegurar estabilidad térmica y ausencia de pánicos de segmentación en hilos de computación intensiva.
