import { z } from "zod";

export enum CategoryType {
  INCOME = "income",
  EXPENSE = "expense",
}

export const createCategorySchema = z.object({
  name: z.string().nonempty(),
  description: z.string().optional(),
  type: z.nativeEnum(CategoryType),
});

export const updateCategorySchema = createCategorySchema.partial();

export type CreateCategoryDto = z.infer<typeof createCategorySchema>;
export type UpdateCategoryDto = z.infer<typeof updateCategorySchema>;
