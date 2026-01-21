# 📘 CONCEPTO: SOVEREIGN IDENTITY PROFILE (V13.0)

**Clasificación:** DOMAIN LOGIC (ESTRATO L2)
**Hito:** V13.0 - Hydra-ID Deployment

## 1. El Salto de Abstracción
La identidad deja de ser un "consumible" (cookie) para convertirse en un "Activo de Infraestructura". El modelo `Identity` ahora encapsula el entorno completo necesario para que una sesión sea indetectable.

## 2. Física del Fingerprint
El campo `browser_fingerprint_json` permite que el sistema almacene la respuesta de las APIs de `Canvas` y `WebGL` del computador real del operador. Cuando un worker efímero carga esta identidad, **suplantará** su propio hardware por estos valores, haciendo que Google perciba un retorno del mismo dispositivo.

## 3. Gobernanza y Leases
La inclusión de `leased_until` en el modelo garantiza que el Dashboard L5 pueda visualizar en tiempo real qué cuentas están "ocupadas" por el enjambre, previniendo la colisión de sesiones que dispararía el baneo por IP duplicada.
