📜 ADICIÓN AL MANIFIESTO: PROTOCOLO HYDRA-ID (V2026.3)
1. Visión del Dispositivo Soberano
Cada identidad en la Bóveda ZK ya no será solo un set de cookies. Se convertirá en un Perfil de Hardware Virtual.
Vínculo Indisoluble: Una cuenta de Google (email) estará atada permanentemente a una Firma de Hardware (Fingerprint) y a una Coordenada de Red (IP/Proxy).
Persistencia de Dispositivo: Al re-utilizar el mismo Canvas ID, WebGL Renderer y AudioContext para la misma cuenta, Google detecta un "dispositivo conocido", lo que reduce drásticamente la probabilidad de desafíos de seguridad (CAPTCHA).
2. El Estrato de Red (Galvanic Proxying)
Se ha diseñado el camino para la inyección de IPs dedicadas.
Mapeo Táctico: La tabla identities se expandirá para incluir proxy_url.
Flujo: Dashboard (Input IP) -> Turso L3 -> Provisioner L6 -> Playwright (Proxy Config).
Resiliencia: Si no se provee una IP comprada, el sistema hará fallback a la IP del Data Center, pero manteniendo el Fingerprint para asegurar la mitad de la identidad.
3. El Pulso Metabólico (Low-Energy Human Trace)
Para mantener los timestamps de las cookies (__Secure-1PSIDTS) frescos sin agotar los recursos de RAM/CPU de Colab:
Acción: Antes de la ignición del minero, el navegador cargará una búsqueda de Google de texto plano (ej: google.com/search?q=cryptographic+audit+status).
Optimización: Se deshabilitará la carga de imágenes, fuentes externas y CSS pesado en esa pestaña de "pulso" para que el consumo de recursos sea < 5% del total de la VM.
