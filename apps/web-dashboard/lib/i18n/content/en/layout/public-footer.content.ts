import { type PublicFooterParams } from "../../../schemas/layout/public-footer.schema";

export const publicFooterContent: PublicFooterParams = {
  brand: {
    mission: "Distributed entropy audit and cryptographic archaeology over the Bitcoin immutable ledger using high-performance ephemeral computing.",
    copyright: "MetaShark Tech. All rights reserved.",
    location: "Global HQ: Santa Catarina, Brazil.",
  },
  columns: {
    product: "Protocol",
    resources: "Knowledge",
    community: "Grid",
    legal: "Sovereignty",
  },
  newsletter: {
    title: "Intelligence Terminal",
    description: "Receive telemetry bursts and updates on weak entropy findings directly to your inbox.",
    placeholder: "operator@metashark.tech",
    button: "SUBSCRIBE",
  },
  disclaimer: "Doctoral academic research tool. MetaShark Tech is not responsible for illicit use of the suite. Accessing third-party private keys contravenes the ethical protocols of the system. Use exclusively for security auditing and ECC research.",
};
