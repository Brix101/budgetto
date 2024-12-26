import { z } from "zod";

export const signInSchema = z.object({
  email: z.string().email(),
  password: z.string().min(8),
});

export const refreshSchema = z.object({
  refreshToken: z.string(),
});

export const signInResponseSchema = z.object({
  accessToken: z.string(),
  refreshToken: z.string(),
  expiresIn: z.number(),
});

export type SignInDto = z.infer<typeof signInSchema>;
export type RefreshDto = z.infer<typeof refreshSchema>;
export type SignInResponseDto = z.infer<typeof signInResponseSchema>;
