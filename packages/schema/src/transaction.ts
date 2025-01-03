import { z } from "zod";

export const createTransactionSchema = z.object({
  amount: z.coerce.number(),
  description: z.string().optional().default(""),
  date: z
    .date()
    .optional()
    .default(() => new Date()),
  categoryId: z.number(),
});

export const updateTransactionSchema = createTransactionSchema.partial();

export type CreateTransactionDto = z.input<typeof createTransactionSchema>;
export type UpdateTransactionDto = z.input<typeof updateTransactionSchema>;
