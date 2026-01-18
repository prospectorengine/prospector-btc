# 📘 CONCEPTO: COLAB CONTROLLER (ORCHESTRATED IGNITION)

## 1. Física del Despliegue
El controlador actúa como el puente entre el plan de misión (C2) y el entorno de ejecución hostil. Su misión es garantizar que el binario de Rust sea inyectado en Colab de forma que Google no pueda distinguir la acción de un usuario real.

## 2. Reducción de Regresiones
- **Alineación Sentinel:** Se corrige el constructor para permitir que cada nodo envíe sus propios logs al Dashboard.
- **Navigator Sync:** Se inyecta la instancia de Sentinel en el Navigator para que la fase de navegación deje de ser una "caja negra".
- **Payload Sync:** Se utiliza el nombre nominal `generate_mission_payload` evitando errores de resolución de módulo.

## 3. Topología de Observabilidad
`ColabController` -> `Sentinel` -> `Orchestrator (L3)` -> `Dashboard (L5)`
Cada método privado informa su progreso, permitiendo ver en Vercel la barra de progreso de la ignición.
