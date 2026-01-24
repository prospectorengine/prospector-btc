# 📘 CONCEPTO: TACTICAL BILLING HISTORY (OUTBOX READ)

**Clasificación:** PERSISTENCIA TÁCTICA (ESTRATO L3)
**Misión:** V1.4 - Visibilidad del Gasto Energético

## 1. El Rol del Outbox como Historial
En el protocolo Hydra-Zero, la tabla `outbox_strategic` no es solo un buffer de transporte; es un **Log de Eventos Inmutable**. Al implementar la lectura del historial sobre esta tabla, permitimos que el operador vea sus transacciones instantáneamente, sin esperar a que el `StrategicRelay` confirme la sincronía con Supabase.

## 2. Isomorfismo JSONB -> Domain
El repositorio actúa como un transductor. Extrae el campo `payload_json` y utiliza `serde_json` para reconstruir la estructura `ComputeCreditTransaction`. Este diseño permite que el esquema del historial sea evolutivo: si añadimos campos en el futuro, solo necesitamos actualizar el modelo de dominio.

## 3. Optimización de Lectura
- **Index Polling:** La consulta utiliza el orden cronológico descendente, lo que en SQLite es extremadamente eficiente al ser la llave de ordenación natural de la inserción.
- **Detección de Deriva:** Si un registro en el Outbox está corrupto (ej: JSON malformado), el motor lo ignora y loguea una alerta para el Panóptico, previniendo que la UI del Dashboard colapse.
