import { z } from "zod";

export const createUserSchema = z.object({
  name: z.string().min(2),
  email: z.string().email(),
  password: z.string().min(8),
});

export const updateUserSchema = createUserSchema.partial();

export const userSchema = z.object({
  id: z.number(),
  name: z.string().optional(),
  email: z.string().optional(),
  createdAt: z.string().date().optional(),
  updatedAt: z.string().date().optional(),
});

export type CreateUserDto = z.infer<typeof createUserSchema>;
export type UpdateUserDto = z.infer<typeof updateUserSchema>;
export type UserSchemaDto = z.infer<typeof userSchema>;
