// INICIO DEL ARCHIVO [apps/web-dashboard/components/system/guided-tour.tsx]
/**
 * =================================================================
 * APARATO: GUIDED TOUR OVERLAY (V2.0 - SPOTLIGHT FX)
 * CLASIFICACIÓN: UX COMPONENT
 * RESPONSABILIDAD: RENDERIZADO DEL SISTEMA DE GUÍA VISUAL
 * =================================================================
 */

"use client";

import { useEffect, useState } from "react";
import { AnimatePresence, motion } from "framer-motion";
import { useTour } from "@/hooks/use-tour";
import { Button } from "@/components/ui/kit/button";
import { ChevronRight, X, Target } from "lucide-react";
import { useTranslations } from "next-intl";

interface TourStep {
  targetId: string;
  titleKey: string;
  descKey: string;
  position: "top" | "bottom" | "left" | "right";
}

// DEFINICIÓN DE LA SECUENCIA DE ENTRENAMIENTO
const TOUR_STEPS: TourStep[] = [
  { targetId: "tour-system-monitor", titleKey: "step1_title", descKey: "step1_desc", position: "bottom" },
  { targetId: "tour-identity-vault", titleKey: "step2_title", descKey: "step2_desc", position: "left" },
  { targetId: "tour-swarm-launcher", titleKey: "step3_title", descKey: "step3_desc", position: "left" },
  { targetId: "tour-audit-trail", titleKey: "step4_title", descKey: "step4_desc", position: "top" },
  { targetId: "tour-integrity-hud", titleKey: "step5_title", descKey: "step5_desc", position: "right" },
];

export function GuidedTour() {
  const { isTourActive, currentStep, nextStep, endTour } = useTour();
  const t = useTranslations("System.tour");
  const [targetRect, setTargetRect] = useState<DOMRect | null>(null);

  // Sincronización con el DOM (Resize Observer)
  useEffect(() => {
    if (!isTourActive) return;

    const updatePosition = () => {
      const step = TOUR_STEPS[currentStep];
      if (!step) {
        endTour();
        return;
      }
      const element = document.getElementById(step.targetId);
      if (element) {
        // Hacemos scroll suave hacia el elemento
        element.scrollIntoView({ behavior: "smooth", block: "center" });
        setTargetRect(element.getBoundingClientRect());
      }
    };

    // Pequeño delay para permitir que el scroll termine o el layout se asiente
    const timeout = setTimeout(updatePosition, 300);
    window.addEventListener("resize", updatePosition);
    window.addEventListener("scroll", updatePosition);

    return () => {
      window.removeEventListener("resize", updatePosition);
      window.removeEventListener("scroll", updatePosition);
      clearTimeout(timeout);
    };
  }, [currentStep, isTourActive, endTour]);

  if (!isTourActive || !targetRect) return null;

  const currentStepData = TOUR_STEPS[currentStep];
  const isLastStep = currentStep === TOUR_STEPS.length - 1;

  // Cálculo de posición del Tooltip
  const tooltipStyle: React.CSSProperties = {
    position: "fixed",
    zIndex: 100,
  };

  if (currentStepData.position === "bottom") {
    tooltipStyle.top = targetRect.bottom + 20;
    tooltipStyle.left = targetRect.left;
  } else if (currentStepData.position === "top") {
    tooltipStyle.bottom = window.innerHeight - targetRect.top + 20;
    tooltipStyle.left = targetRect.left;
  } else if (currentStepData.position === "left") {
    tooltipStyle.top = targetRect.top;
    tooltipStyle.right = window.innerWidth - targetRect.left + 20;
  } else if (currentStepData.position === "right") {
    tooltipStyle.top = targetRect.top;
    tooltipStyle.left = targetRect.right + 20;
  }

  return (
    <div className="fixed inset-0 z-[90] pointer-events-none">
      {/* CAPA OSCURA CON RECORTE (MASK) */}
      <svg className="absolute inset-0 w-full h-full text-black/80 fill-current">
        <defs>
          <mask id="spotlight-mask">
            <rect className="w-full h-full fill-white" />
            <rect
              x={targetRect.left - 10}
              y={targetRect.top - 10}
              width={targetRect.width + 20}
              height={targetRect.height + 20}
              rx="12"
              className="fill-black"
            />
          </mask>
        </defs>
        <rect className="w-full h-full" mask="url(#spotlight-mask)" />
      </svg>

      {/* MARCO DE RESALTE ANIMADO */}
      <motion.div
        layoutId="tour-highlight"
        className="absolute border-2 border-emerald-500 rounded-xl shadow-[0_0_50px_rgba(16,185,129,0.5)] bg-transparent pointer-events-none"
        style={{
          top: targetRect.top - 10,
          left: targetRect.left - 10,
          width: targetRect.width + 20,
          height: targetRect.height + 20,
        }}
        initial={false}
        transition={{ type: "spring", stiffness: 300, damping: 30 }}
      >
        <div className="absolute -top-3 -right-3 bg-emerald-500 text-black text-[10px] font-black px-2 py-0.5 rounded-full uppercase tracking-widest">
          Step {currentStep + 1}/{TOUR_STEPS.length}
        </div>
      </motion.div>

      {/* TARJETA DE INSTRUCCIÓN (MINI CARD) */}
      <AnimatePresence mode="wait">
        <motion.div
          key={currentStep}
          initial={{ opacity: 0, y: 10, scale: 0.95 }}
          animate={{ opacity: 1, y: 0, scale: 1 }}
          exit={{ opacity: 0, scale: 0.95 }}
          className="bg-[#0f0f0f] border border-zinc-700 p-6 rounded-2xl shadow-2xl pointer-events-auto max-w-sm w-full backdrop-blur-xl"
          style={tooltipStyle}
        >
          <div className="flex items-start gap-4">
            <div className="p-3 bg-emerald-500/10 rounded-xl border border-emerald-500/20 text-emerald-500 shrink-0">
              <Target className="w-6 h-6" />
            </div>
            <div className="space-y-2">
              <h3 className="text-sm font-black text-white uppercase tracking-wider font-mono">
                {t(currentStepData.titleKey)}
              </h3>
              <p className="text-xs text-zinc-400 leading-relaxed font-mono">
                {t(currentStepData.descKey)}
              </p>
            </div>
          </div>

          <div className="flex items-center justify-between mt-6 pt-4 border-t border-white/5">
            <button
              onClick={endTour}
              className="text-[10px] text-zinc-500 hover:text-white uppercase font-bold tracking-widest transition-colors flex items-center gap-1"
            >
              <X className="w-3 h-3" /> {t("skip")}
            </button>

            <Button variant="cyber" size="sm" onClick={nextStep} className="h-8 text-[10px]">
              {isLastStep ? t("finish") : t("next")}
              <ChevronRight className="w-3 h-3 ml-1" />
            </Button>
          </div>
        </motion.div>
      </AnimatePresence>
    </div>
  );
}
// FIN DEL ARCHIVO [apps/web-dashboard/components/system/guided-tour.tsx]
