// INICIO DEL ARCHIVO [apps/web-dashboard/lib/i18n/content/en/pages/landing.content.ts]
import { type LandingPageParams } from "../../../schemas/pages/landing.schema";

export const landingPageContent = {
  meta: {
    title: "Prospector // Hydra-Zero Protocol",
    description: "Distributed Cryptographic Audit System targeting secp256k1.",
  },
  hero: {
    badge: "SYSTEM OPERATIONAL // V11.5",
    title: "Distributed Entropy Archaeology",
    subtitle:
      "Join the global swarm. Auditing the immutable ledger through probabilistic mathematics and ephemeral computing.",
    cta_primary: {
      label: "Initialize System",
      tooltip: "Start Free Tier",
    },
  },
  capsules: {
    login: {
      title: "Active Operators",
      description: "Access Mission Control dashboard via secure handshake.",
      cta: "ACCESS CONSOLE",
      badge: "SECURE",
    },
    register: {
      title: "New Deployment",
      description: "Provision a new node identity and join the research grid.",
      cta: "INITIALIZE PROTOCOL",
      badge: "OPEN",
    },
  },
  pricing: {
    observer_title: "Observer Node",
    observer_desc: "Real-time visibility into global swarm metrics and keyspace coverage telemetry.",
    cta_free: "START MONITORING",
    operator_title: "Operator Node",
    operator_desc: "Full control over node provisioning, identity injection, and forensic lab access.",
    cta_pro: "UPGRADE PROTOCOL",
  },
} satisfies LandingPageParams;
// FIN DEL ARCHIVO [apps/web-dashboard/lib/i18n/content/en/pages/landing.content.ts]
