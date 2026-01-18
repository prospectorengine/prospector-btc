/**
 * APARATO: NODE FINGERPRINT GENERATOR
 * CLASIFICACIÓN: ESTRATO L1 - HARDWARE IDENTITY
 */
export function generateNodeFingerprint(): string {
  const canvas = document.createElement('canvas');
  const gl = canvas.getContext('webgl');
  // Extraemos info de la GPU y pantalla sin alertar al usuario
  const debugInfo = gl?.getExtension('WEBGL_debug_renderer_info');
  const renderer = gl?.getParameter(debugInfo?.UNMASKED_RENDERER_WEBGL || gl.RENDERER);

  const rawId = [
    navigator.userAgent,
    renderer,
    screen.width + 'x' + screen.height,
    new Date().getTimezoneOffset(),
    navigator.hardwareConcurrency // Núcleos de CPU
  ].join('|');

  // Camuflamos el ID como un "Checksum de Integridad de Memoria"
  return btoa(rawId).substring(0, 32);
}
