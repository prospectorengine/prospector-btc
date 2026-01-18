import { type ScriptsAtom } from "../../../../schemas/dashboard/atoms/scripts.schema";

export const scriptsContent: ScriptsAtom = {
  page_title: "Tactical Command Deck",
  page_subtitle: "Matriz de ejecución de scripts de bajo nivel para el mantenimiento y auditoría del enjambre.",
  labels: {
    copy_cmd: "COPIAR_COMANDO",
    risk: "RIESGO",
    stratum: "ESTRATO"
  },
  definitions: {
    db_migrate: {
      label: "Migrar Base de Datos",
      desc: "Sincroniza el esquema estructural V151.0 con el cluster de Turso (Motor A). Obligatorio tras cambios en L3."
    },
    audit_health: {
      label: "Auditoría de Salud ZK",
      desc: "Ejecuta el descifrado masivo de la bóveda para detectar cookies caducadas y factores de riesgo en las identidades."
    },
    purge_github: {
      label: "Aniquilador C2",
      desc: "Protocolo de emergencia. Cancela recursivamente todos los workflows en cola y limpia el historial de GitHub Actions."
    },
    i18n_gen: {
      label: "Generador de Idiomas",
      desc: "Compila y valida la paridad de los diccionarios EN/ES contra el esquema Zod de la Fuente Única de Verdad."
    },
    build_miner: {
      label: "Forja de Binario Estático",
      desc: "Compila el motor de minería Rust con target x86_64-unknown-linux-musl para portabilidad absoluta en Colab."
    }
  }
};
