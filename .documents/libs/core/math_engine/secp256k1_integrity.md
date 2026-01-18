# 📘 CONCEPTO: CERTIFICADOR GEOMÉTRICO

**Módulo Objetivo:** `libs/core/math-engine/src/secp256k1.rs`

## 1. Física del Aparato
Este test valida la implementación de la ley de grupo sobre la curva de Bitcoin $y^2 = x^3 + 7 \pmod p$. Se enfoca en las Coordenadas Jacobianas para asegurar que el sistema puede sumar puntos sin realizar inversiones modulares costosas en cada paso.

## 2. Justificación Técnica
- **Vectores Génesis:** Se utilizan los valores de $G$ y $2G$ extraídos del código original de Satoshi (2009) para garantizar que el motor es compatible con el rastro de la Blockchain.
- **Identidad Proyectiva:** Se certifica que las proyecciones de retorno al plano afín (X/Z², Y/Z³) mantienen la precisión total de 256 bits.
- **Rendimiento:** La velocidad de adición de puntos es el factor limitante del hashrate global. Este test mide la eficiencia del `Hot-Path` geométrico.


---

Ubicación: .documents/libs/core/math_engine/secp256k1_integrity_v22.md
Física del Aparato: Este test certifica la arquitectura de leyes de grupo en el espacio proyectivo. Valida que las fórmulas de adición Jacobiana optimizadas para
a
=
0
a=0
 (secp256k1) son bit-perfectas.
Mecánica de Duplicación: Se centra en el algoritmo de duplicación Jacobiana (
3
M
+
4
S
3M+4S
), garantizando que el punto
2
G
2G
 resultante sea el ancla correcta para todas las misiones secuenciales.
Topología Neural: El test actúa como un agente C2. Al finalizar, inyecta su ProvingReport en el orquestador, alimentando la nueva UI de Proving Grounds del Dashboard Next.js.

---
