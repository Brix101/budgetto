import { z } from "zod";

export const createTransactionSchema = z.object({
  amount: z.coerce.number(),
  description: z.string().optional(),
  date: z.date().optional(),
  categoryId: z.number(),
});

export const updateTransactionSchema = createTransactionSchema.partial();

export type CreateTransactionDto = z.input<typeof createTransactionSchema>;
export type UpdateTransactionDto = z.input<typeof updateTransactionSchema>;
