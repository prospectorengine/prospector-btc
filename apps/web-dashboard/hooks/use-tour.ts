// INICIO DEL ARCHIVO [apps/web-dashboard/hooks/use-tour.ts]
/**
 * =================================================================
 * APARATO: TACTICAL TOUR ENGINE (V1.0)
 * CLASIFICACIÓN: UX STATE MANAGER
 * RESPONSABILIDAD: ORQUESTACIÓN DE LA GUÍA INTERACTIVA
 * =================================================================
 */

import { create } from "zustand";
import { persist } from "zustand/middleware";

interface TourState {
  isTourActive: boolean;
  currentStep: number;
  hasCompletedTour: boolean;
  startTour: () => void;
  endTour: () => void;
  nextStep: () => void;
  prevStep: () => void;
  resetTour: () => void;
}

export const useTour = create<TourState>()(
  persist(
    (set) => ({
      isTourActive: false,
      currentStep: 0,
      hasCompletedTour: false,

      startTour: () => set({ isTourActive: true, currentStep: 0 }),

      endTour: () => set({ isTourActive: false, hasCompletedTour: true }),

      nextStep: () => set((state) => ({ currentStep: state.currentStep + 1 })),

      prevStep: () => set((state) => ({ currentStep: Math.max(0, state.currentStep - 1) })),

      resetTour: () => set({ hasCompletedTour: false, currentStep: 0, isTourActive: true }),
    }),
    {
      name: "hydra_tour_persistence_v1", // Clave en LocalStorage
      partialize: (state) => ({ hasCompletedTour: state.hasCompletedTour }), // Solo persistimos si ya lo completó
    }
  )
);
// FIN DEL ARCHIVO [apps/web-dashboard/hooks/use-tour.ts]
