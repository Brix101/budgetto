import { z } from "zod";

import { userSchema } from "./user";

export const signInSchema = z.object({
  email: z.string().email({ message: "Invalid email address." }),
  password: z
    .string()
    .min(8, { message: "Password must be at least 8 characters long." }),
});

export const refreshSchema = z.object({
  refreshToken: z.string(),
});

export const signInResponseSchema = z.object({
  user: userSchema,
  accessToken: z.string(),
  refreshToken: z.string(),
  expiresIn: z.number(),
});

export type SignInDto = z.infer<typeof signInSchema>;
export type RefreshDto = z.infer<typeof refreshSchema>;
export type SignInResponseDto = z.infer<typeof signInResponseSchema>;
