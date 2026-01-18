# 📘 CONCEPTO: COLAB NAVIGATOR (TELEMETRY-LINKED)

## 1. Física de la Navegación
El navegador no solo manipula el DOM, sino que actúa como una **Célula de Inteligencia**. Cada decisión (clic, espera, error) genera una traza que alimenta la consola de Vercel.

## 2. Redundancia de Selectores
El aparato utiliza un bucle de reintento sobre `SELECTORS.RUNTIME.CONNECT_BTN`. Esto previene fallos ante actualizaciones A/B de la interfaz de Google, garantizando que el provisioner sea "Maintenance-Free" por períodos prolongados.

## 3. Integración Sentinel
Al sustituir el `prefix` manual por la instancia de `Sentinel`, el navegador gana la capacidad de emitir logs estructurados (JSON) que el Orquestador traduce a eventos SSE (`t: "pl"`).

---


