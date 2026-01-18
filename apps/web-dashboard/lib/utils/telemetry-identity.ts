/**
 * APARATO: DETERMINISTIC HARDWARE IDENTITY
 * CLASIFICACIÓN: ESTRATO L1 - IDENTITY CORE
 */
export function generateMachineHardwareFingerprint(): string {
  if (typeof window === "undefined") return "SERVER_NODE";

  const canvas = document.createElement("canvas");
  const gl = canvas.getContext("webgl");
  const debugInfo = gl?.getExtension("WEBGL_debug_renderer_info");
  const renderer = gl?.getParameter(debugInfo?.UNMASKED_RENDERER_WEBGL || gl.RENDERER) || "UNKNOWN_RENDERER";

  const screenMetrics = `${window.screen.width}x${window.screen.height}x${window.screen.colorDepth}`;
  const coreContext = `${navigator.hardwareConcurrency || 0}|${navigator.language}|${renderer}`;

  // Generamos un hash base64 que parezca un Checksum de integridad técnica
  const rawSignature = `HYDRA-SIG|${coreContext}|${screenMetrics}`;
  return btoa(rawSignature).substring(0, 48);
}
