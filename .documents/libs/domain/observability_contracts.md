# 📘 CONCEPTO: CONTRATOS DE OBSERVABILIDAD C2

## 1. Física del Aparato
El sistema de mando requiere saber no solo si un worker está minando, sino qué está haciendo el **automatizador** (L6) antes de que el worker nazca. Este contrato habilita el "paso de mensajes" desde el entorno efímero de GitHub Actions hasta el Dashboard.

## 2. Lógica del Ban-Shield
Para evitar el baneo de cuentas de Google, implementamos una validación de capacidad:
- **Ratio de Seguridad:** 1 identidad (cuenta) : 3 nodos simultáneos.
- **Autorización:** El Orquestador niega la señal de ignición si `requested_nodes > (identities * 3)`.

## 3. Topología
`Provisioner (L6)` -> `Orchestrator (L3)` -> `SSE Stream` -> `Dashboard (L5)`

---


