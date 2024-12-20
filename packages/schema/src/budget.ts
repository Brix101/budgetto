import { z } from "zod";

export const createBudgetSchema = z.object({
  amount: z.coerce.number(),
  description: z.string(),
  startDate: z.date(),
  endDate: z.date(),
  categoryId: z.coerce.number(),
});

export const updateBudgetSchema = createBudgetSchema.partial();

export type CreateBudgetDto = z.infer<typeof createBudgetSchema>;
export type UpdateBudgetDto = z.infer<typeof updateBudgetSchema>;
