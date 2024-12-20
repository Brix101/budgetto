import { z } from "zod";

export enum CategoryType {
  INCOME = "income",
  EXPENSE = "expense",
}

export const createCategorySchema = z.object({
  name: z.string().min(2),
  email: z.string().email(),
  password: z.string().min(8),
});
