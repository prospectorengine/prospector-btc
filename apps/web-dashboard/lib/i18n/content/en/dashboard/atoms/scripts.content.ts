/**
 * =================================================================
 * APARATO: SCRIPTS I18N CONTENT (EN - V1.0)
 * CLASIFICACIÓN: I18N ATOM (ESTRATO L5)
 * RESPONSABILIDAD: TECHNICAL DESCRIPTIONS FOR COMMAND DECK
 * =================================================================
 */

import { type ScriptsAtom } from "../../../../schemas/dashboard/atoms/scripts.schema";

export const scriptsContentEn: ScriptsAtom = {
  page_title: "Tactical Command Deck",
  page_subtitle: "Execution matrix for low-level scripts, swarm maintenance, and cryptographic audit tools.",
  labels: {
    copy_cmd: "COPY_COMMAND",
    risk: "RISK_LEVEL",
    stratum: "STRATUM"
  },
  definitions: {
    db_migrate: {
      label: "Migrate Database",
      desc: "Synchronizes the structural schema V151.0 with the Turso cluster (Engine A). Mandatory after L3 strata changes."
    },
    audit_health: {
      label: "ZK Health Audit",
      desc: "Executes mass vault decryption to detect expired cookies and risk factors within active identities."
    },
    purge_github: {
      label: "C2 Annihilator",
      desc: "Emergency protocol. Recursively cancels all queued workflows and incinerates GitHub Actions history."
    },
    i18n_gen: {
      label: "I18n Compiler",
      desc: "Validates and compiles EN/ES dictionary parity against the Zod schema of the Single Source of Truth."
    },
    build_miner: {
      label: "Static Binary Forge",
      desc: "Compiles the Rust engine with target x86_64-unknown-linux-musl for absolute portability in Google Colab."
    }
  }
};
