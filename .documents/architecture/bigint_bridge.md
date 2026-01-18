# 📘 CONCEPTO: THE BIGINT BRIDGE

**Problema:** JavaScript `Number` (IEEE 754) pierde precisión por encima de $2^{53}$ (9 Peta).
**Contexto:**
- Hashrates globales pueden superar 9 PH/s.
- Timestamps en nanosegundos desbordan.
- Aritmética de 64 bits de Rust (`u64`) llega a 18 Exa.

**Solución Sincronizada:**
1.  **Backend (Rust):** Mantiene `u64` para aritmética nativa de CPU (máxima velocidad).
2.  **Frontera (Typeshare):** Se instruye al generador para emitir estos campos como `string` en TypeScript (`#[typeshare(serialized_as = "String")]`).
3.  **Frontend (TypeScript):** Recibe strings. Si necesita operar matemáticamente, debe castear a `BigInt("1234...")`. Para visualización, se formatea directamente.

Esto garantiza integridad de datos al 100% entre el Motor L1 y el Dashboard L5.

---

