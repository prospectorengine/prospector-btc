import { z } from "zod";

export const PublicFooterSchema = z.object({
  brand: z.object({
    mission: z.string(),
    copyright: z.string(),
    location: z.string(),
  }),
  columns: z.object({
    product: z.string(),
    resources: z.string(),
    community: z.string(),
    legal: z.string(),
  }),
  newsletter: z.object({
    title: z.string(),
    description: z.string(),
    placeholder: z.string(),
    button: z.string(),
  }),
  disclaimer: z.string(),
});

export type PublicFooterParams = z.infer<typeof PublicFooterSchema>;
