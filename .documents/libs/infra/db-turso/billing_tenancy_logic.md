# 📘 CONCEPTO: BILLING MULTI-TENANCY (JSONB EXTRACTION)

**Clasificación:** PERSISTENCIA TÁCTICA (ESTRATO L3)
**Misión:** V1.5 - Aislamiento de Valor Usuario

## 1. El Desafío del Outbox Compartido
En la arquitectura de Motores Gemelos, la tabla `outbox_strategic` es un sumidero común para todos los eventos del sistema. Sin una estrategia de filtrado molecular, cualquier consulta al historial de facturación expondría los consumos de otros operadores.

## 2. La Solución: Inspección Molecular (json_extract)
Dado que el `operator_id` reside dentro de la columna `payload_json`, el repositorio utiliza la capacidad nativa de libSQL/SQLite para indexar y buscar sobre el contenido del documento.
$$ \text{Query} = \text{Outbox} \cap \text{Stratum}_{\text{Billing}} \cap \text{Payload.operator\_id} $$

## 3. Integridad del Reporte
Al inyectar el `operator_id` en la fase de `queue_credit_deduction`, garantizamos que la "Cadena de Custodia Energética" sea inquebrantable desde el momento en que se adquiere la misión hasta que se visualiza en el Dashboard Zenith.
