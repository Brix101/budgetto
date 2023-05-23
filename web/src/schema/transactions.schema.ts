import z from "zod";

export const categorySchema = z.object({
  id: z.string(),
  name: z.string(),
  note: z.string().nullish(),
  isDefault: z.boolean(),
});

export const categoriesSchema = z.object({
  categories: categorySchema.array(),
});

export type Category = z.TypeOf<typeof categorySchema>;
export type Categories = z.TypeOf<typeof categoriesSchema>;
