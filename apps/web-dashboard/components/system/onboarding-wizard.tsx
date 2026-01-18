// INICIO DEL ARCHIVO [apps/web-dashboard/components/system/onboarding-wizard.tsx]
/**
 * =================================================================
 * APARATO: ONBOARDING WIZARD (V1.1 - ANIMATION FIXED)
 * CLASIFICACIÓN: UX SYSTEM (ESTRATO L5)
 * RESPONSABILIDAD: EDUCACIÓN INICIAL DEL OPERADOR HYDRA
 * =================================================================
 */

"use client";

import React, { useState, useEffect } from "react";
import { motion, AnimatePresence } from "framer-motion";
import {
  X,
  ChevronRight,
  ChevronLeft,
  Zap,
  ShieldCheck,
  Terminal,
  Cpu
} from "lucide-react";
import { Button } from "@/components/ui/kit/button";

interface Step {
  title: string;
  description: string;
  icon: React.ElementType;
}

export function OnboardingWizard() {
  const [isOpen, setIsOpen] = useState(false);
  const [currentStep, setCurrentStep] = useState(0);

  // Pasos definidos estáticamente
  const steps: Step[] = [
    {
      title: "Bienvenido al Protocolo Hydra-Zero",
      description: "Has accedido al sistema de auditoría distribuida más avanzado. Esta terminal te permite coordinar un enjambre de minería forense sobre la curva secp256k1.",
      icon: Terminal
    },
    {
      title: "Centro de Mando",
      description: "El Dashboard es tu HUD principal. Monitorea la latencia de la red, el hashrate global del enjambre y las colisiones criptográficas en tiempo real.",
      icon: Cpu
    },
    {
      title: "Bóveda de Identidad (ZK-Vault)",
      description: "Antes de lanzar una misión, debes inyectar credenciales (Cookies) en la Bóveda. Usamos cifrado AES-256-GCM en tu navegador. Tus llaves nunca tocan nuestros servidores sin cifrar.",
      icon: ShieldCheck
    },
    {
      title: "Lanzamiento de Misiones",
      description: "Usa el 'Swarm Launcher' para desplegar nodos en Google Colab o Kaggle. El sistema automatizará la búsqueda y reportará hallazgos aquí.",
      icon: Zap
    }
  ];

  useEffect(() => {
    const hasSeenTutorial = localStorage.getItem("hydra_v1_onboarding_completed");
    if (!hasSeenTutorial) {
      const timer = setTimeout(() => setIsOpen(true), 1500);
      return () => clearTimeout(timer);
    }
  }, []);

  const handleNext = () => {
    if (currentStep < steps.length - 1) {
      setCurrentStep(prev => prev + 1);
    } else {
      handleClose();
    }
  };

  const handlePrev = () => {
    if (currentStep > 0) {
      setCurrentStep(prev => prev - 1);
    }
  };

  const handleClose = () => {
    setIsOpen(false);
    localStorage.setItem("hydra_v1_onboarding_completed", "true");
  };

  return (
    <AnimatePresence>
      {isOpen && (
        <div className="fixed inset-0 z-[100] flex items-center justify-center p-4">
          <motion.div
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            exit={{ opacity: 0 }}
            className="absolute inset-0 bg-black/80 backdrop-blur-md"
            onClick={handleClose}
          />

          <motion.div
            initial={{ opacity: 0, scale: 0.9, y: 20 }}
            animate={{ opacity: 1, scale: 1, y: 0 }}
            exit={{ opacity: 0, scale: 0.9, y: 20 }}
            className="relative w-full max-w-lg bg-[#0A0A0A] border border-emerald-500/30 rounded-2xl shadow-[0_0_50px_rgba(16,185,129,0.1)] overflow-hidden"
          >
            {/* Progress Bar */}
            <div className="h-1 w-full bg-zinc-900">
              <motion.div
                className="h-full bg-emerald-500 shadow-[0_0_10px_#10b981]"
                initial={{ width: 0 }}
                animate={{ width: `${((currentStep + 1) / steps.length) * 100}%` }}
                transition={{ duration: 0.5 }}
              />
            </div>

            <div className="p-8">
              <div className="flex justify-between items-start mb-6">
                <div className="p-3 bg-emerald-500/10 rounded-xl border border-emerald-500/20 text-emerald-500">
                  {React.createElement(steps[currentStep].icon, { className: "w-8 h-8" })}
                </div>
                <button
                  onClick={handleClose}
                  className="text-zinc-500 hover:text-white transition-colors"
                >
                  <X className="w-5 h-5" />
                </button>
              </div>

              <div className="space-y-4 mb-8 min-h-[140px]">
                <AnimatePresence mode="wait">
                  <motion.div
                    key={currentStep}
                    initial={{ opacity: 0, x: 20 }}
                    animate={{ opacity: 1, x: 0 }}
                    exit={{ opacity: 0, x: -20 }}
                    transition={{ duration: 0.3 }}
                  >
                    <h2 className="text-xl font-black text-white uppercase tracking-wider font-mono mb-3">
                      {steps[currentStep].title}
                    </h2>
                    <p className="text-sm text-zinc-400 font-mono leading-relaxed">
                      {steps[currentStep].description}
                    </p>
                  </motion.div>
                </AnimatePresence>
              </div>

              <div className="flex items-center justify-between pt-6 border-t border-white/5">
                <div className="flex gap-1">
                  {steps.map((_, idx) => (
                    <div
                      key={idx}
                      className={`h-1.5 w-1.5 rounded-full transition-colors ${idx === currentStep ? 'bg-emerald-500' : 'bg-zinc-800'}`}
                    />
                  ))}
                </div>

                <div className="flex gap-3">
                  <Button
                    variant="ghost"
                    onClick={handlePrev}
                    disabled={currentStep === 0}
                    className="text-zinc-400 hover:text-white"
                  >
                    <ChevronLeft className="w-4 h-4" />
                  </Button>
                  <Button
                    variant="cyber"
                    onClick={handleNext}
                    className="px-6 h-10 text-xs"
                  >
                    {currentStep === steps.length - 1 ? "INICIALIZAR SISTEMA" : "SIGUIENTE"}
                    {currentStep < steps.length - 1 && <ChevronRight className="w-4 h-4 ml-2" />}
                  </Button>
                </div>
              </div>
            </div>
          </motion.div>
        </div>
      )}
    </AnimatePresence>
  );
}
// FIN DEL ARCHIVO [apps/web-dashboard/components/system/onboarding-wizard.tsx]
