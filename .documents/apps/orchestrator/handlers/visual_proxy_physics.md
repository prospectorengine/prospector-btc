# 📘 CONCEPTO: VISUAL PROXY PHYSICS (L3 -> L5)

**Clasificación:** INFRASTRUCTURE GATEWAY (ESTRATO L3)
**Misión:** V86.1 - Sincronía del Panóptico

## 1. El Problema del Aislamiento
Los workers en Google Colab no son accesibles desde el Dashboard por seguridad de red (NAT/Firewall). El Orquestador debe actuar como un **Relé de Imagen** (Visual Relay).

## 2. Flujo de Datos
1. **L6 (Provisioner):** Captura el frame mediante Playwright.
2. **L4 (Transport):** Envía un `POST` JSON con la imagen comprimida en Base64.
3. **L3 (Ingestor):** Valida la identidad del nodo y almacena el frame en un `RwLock<HashMap>`.
4. **L4 (Broadcast):** Emite una señal por el `EventBus` para que los Dashboards conectados actualicen su frame.

## 3. Optimización Térmica
Para evitar el colapso del Orquestador por ráfagas de video:
- Los frames se limitan a 1 por minuto por nodo.
- El `ReaperService` purga los frames de la RAM si el nodo está inactivo > 300s.
