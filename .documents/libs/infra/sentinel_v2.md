// INICIO DEL ARCHIVO [.documents/libs/infra/sentinel_v2.md]
# 📘 CONCEPTO: SENTINEL TELEMETRY BUFFER V2

## 1. Física del Problema
El arranque de nodos en entornos efímeros (Google Colab, Kaggle) es caótico. La red puede estar saturada o bloqueada intermitentemente durante la carga del navegador.
El modelo anterior (`v1`) disparaba una petición HTTP por cada log. Si fallaba, el log se perdía para siempre, dejando al operador "ciego" en el Dashboard.

## 2. Solución: Cola Asíncrona con Backpressure
El nuevo `Sentinel V6.0` implementa un buffer FIFO (`First-In, First-Out`).
1.  **Ingesta:** `emitTrace()` empuja el log a un array en memoria RAM local.
2.  **Procesador:** Un método recursivo `attempt_buffer_flush()` intenta vaciar la cola.
3.  **Resiliencia:** Si `axios.post` falla, incrementa un contador de `consecutive_failures` y programa un reintento con **Backoff Exponencial** (espera más tiempo cuanto más falla).
4.  **Consistencia:** Nunca bloquea el hilo principal de Playwright (`await` no espera la red, solo el encolado).

## 3. Impacto en Observabilidad
Garantiza que la secuencia `VM_READY` -> `INJECTING_PAYLOAD` -> `MINER_STARTED` llegue completa al Dashboard, incluso si llegan con unos segundos de retraso.

---
