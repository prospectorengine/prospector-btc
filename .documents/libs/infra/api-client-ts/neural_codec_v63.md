// INICIO DEL ARCHIVO [.documents/libs/infra/api-client-ts/neural_codec_v63.md]
# 📘 CONCEPTO: NEURAL LINK CODEC V63 (UNIVERSAL)

## 1. Física del Aparato
El `NeuralCodec` es el traductor binario del sistema. Convierte el flujo de datos comprimido (MessagePack) que viaja por el túnel SSE en objetos JavaScript tipados.
En su versión V63, implementa una estrategia de **Detección de Entorno** para resolver la decodificación Base64.

## 2. El Problema de `window.atob`
El método `atob` fue históricamente exclusivo del navegador. Aunque Node.js lo incorporó recientemente en el espacio global, depender de `window` explícitamente rompe la compatibilidad con:
1.  **Server-Side Rendering (SSR):** Next.js pre-renderiza componentes en el servidor.
2.  **Testing (Jest):** Los entornos de prueba a menudo simulan el DOM pero pueden tener discrepancias.
3.  **Edge Runtime:** Entornos serverless ligeros.

## 3. Solución Isomórfica
El aparato ahora consulta `globalThis`, que es el estándar ECMAScript para acceder al objeto global independientemente del entorno (Window en browser, Global en Node, Self en Workers).

$$ Decodificación = MessagePack(Base64_{Universal}(Payload)) $$

Esta abstracción garantiza que el Dashboard pueda procesar telemetría tanto en el cliente como en el servidor sin fricción.


---


