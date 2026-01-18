// INICIO DEL ARCHIVO [apps/web-dashboard/lib/i18n/content/en/dashboard/atoms/academy.content.ts]
import { type AcademyAtom } from "../../../../schemas/dashboard/atoms/academy.schema";

export const academyContent: AcademyAtom = {
  page_title: "Prospector Academy",
  page_subtitle: "Advanced Cryptography & Forensic Analysis Curriculum. Master the protocol.",
  levels: {
    foundation: "FOUNDATION",
    intermediate: "INTERMEDIATE",
    elite: "ELITE OPERATIONS",
  },
  actions: {
    start: "INITIALIZE MODULE",
    continue: "RESUME UPLINK",
    locked: "ENCRYPTED / LOCKED",
    completed: "CERTIFIED",
  },
  modules: {
    mod_01: {
      title: "Elliptic Curve Genesis",
      desc: "Mathematical fundamentals of secp256k1. Understanding the Discrete Logarithm Problem and the geometry of Bitcoins security.",
    },
    mod_02: {
      title: "Entropy Archaeology",
      desc: "Forensic analysis of historical PRNG failures. Dissecting the Debian 2008 and Android 2013 randomness vectors.",
    },
    mod_03: {
      title: "Hydra Swarm Operations",
      desc: "Command and Control (C2) protocols. Managing distributed ephemeral nodes across hostile environments.",
    },
    mod_04: {
      title: "Zero-Knowledge Identity",
      desc: "Advanced security protocols. Client-side AES-256-GCM injection and identity hygiene management.",
    },
  },
};
// FIN DEL ARCHIVO
