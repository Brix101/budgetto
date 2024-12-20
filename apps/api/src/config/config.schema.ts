import { z } from "zod";

const configSchema = z.object({
  NODE_ENV: z
    .enum(["development", "production", "test"])
    .default("development"),
  PORT: z.coerce.number().default(5000),
  DATABASE_HOST: z.string(),
  DATABASE_PORT: z.coerce.number().default(5432),
  DATABASE_USER: z.string(),
  DATABASE_PASSWORD: z.string(),
  DATABASE_NAME: z.string(),
  //   REDIS_HOST: z.string(),
  //   REDIS_PORT: z.coerce.number().default(6379),
  //   REDIS_USER: z.string().optional(),
  //   REDIS_PASSWORD: z.string().optional(),
  //   REDIS_SSL_ENABLED: z.boolean().default(false),
});

export type ConfigSchema = z.infer<typeof configSchema>;

export default configSchema;
