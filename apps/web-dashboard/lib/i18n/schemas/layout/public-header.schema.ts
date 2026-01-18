import { z } from "zod";

export const PublicHeaderSchema = z.object({
  brand: z.string(),
  nav: z.object({
    features: z.string(),
    pricing: z.string(),
    about: z.string(),
  }),
  actions: z.object({
    login: z.string(),
    get_started: z.string(),
  }),
});

export type PublicHeaderParams = z.infer<typeof PublicHeaderSchema>;
