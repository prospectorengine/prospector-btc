import "@testing-library/jest-dom";

/**
 * APARATO: JEST SETUP (V2.1 - ESM FIXED)
 * RESPONSABILIDAD: INYECCIÓN DE POLYFILLS PARA EL ENTORNO DE PRUEBAS
 */

async function injectGlobalPolyfills() {
  if (typeof global.TextEncoder === "undefined") {
    // Utilizando import dinámico para satisfacer @typescript-eslint/no-require-imports
    const { TextEncoder, TextDecoder } = await import("util");
    global.TextEncoder = TextEncoder as unknown as typeof global.TextEncoder;
    global.TextDecoder = TextDecoder as unknown as typeof global.TextDecoder;
  }
}

injectGlobalPolyfills();
