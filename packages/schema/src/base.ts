import z from "zod";

export const baseSchema = z.object({
  id: z.number(),
  createdAt: z.string(),
  updatedAt: z.string(),
});
