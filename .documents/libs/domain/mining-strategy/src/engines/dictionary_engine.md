# 📘 CONCEPTO: ENTROPY DICTIONARY ENGINE

**Clasificación:** Aparato de Estrategia L2 (Humint Vector)
**Estado:** V30.0 (Trinity Compliant)

## 1. Física del Aparato
Este motor explota la falibilidad de la entropía humana. A diferencia de los motores matemáticos (Secuencial/Canguro) que atacan la curva, este motor ataca la **fuente** de la clave.
Se basa en la premisa de que los usuarios tempranos de Bitcoin (2009-2011) generaron claves privadas utilizando el hash SHA-256 de frases memorables ("Brainwallets") en lugar de RNGs criptográficamente seguros.

$$ PrivateKey = SHA256(UTF8("correct horse battery staple")) $$

## 2. Topología y Relaciones
- **Input:** Un vector de cadenas de texto (Diccionario/Permutaciones).
- **Transformación:** `libs/domain/mining-strategy/src/brainwallet.rs`.
- **Validación:** `libs/core/probabilistic/sharded.rs` (Filtro de Bloom O(1)).
- **Output:** Colisiones reportadas vía `FindingHandler`.

## 3. Optimización "Zero-Allocation Hash"
Para maximizar el rendimiento (Throughput), el motor implementa una estrategia de "Hash Crudo":
1.  Genera la Clave Privada (SHA-256).
2.  Deriva la Clave Pública (secp256k1).
3.  Genera el Hash160 (`RIPEMD160(SHA256(PubKey))`).
4.  **CRÍTICO:** Verifica este hash crudo `[u8; 20]` contra el filtro de Bloom.
5.  **Solo si hay colisión**, incurre en el costo de asignar memoria para generar la dirección Base58 (String) legible para el reporte.

Esto reduce la presión sobre el Allocator del sistema en un 99.9% de los casos (Misses).

## 4. Justificación de Diseño
- **Batch Processing:** Aunque procesa línea por línea, está diseñado para ser alimentado por iteradores de alto rendimiento (Memory Mapped Files) en el `executor.rs`.
- **Deterministic Checkpoint:** Retorna el último índice procesado para permitir la reanudación precisa ante interrupciones de `preemption` en la nube.

---

