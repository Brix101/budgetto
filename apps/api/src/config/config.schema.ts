import { z } from "zod";

const configSchema = z.object({
  NODE_ENV: z
    .enum(["development", "production", "test"])
    .default("development"),
  PORT: z.coerce.number().default(5000),
  DB_HOST: z.string(),
  DB_PORT: z.coerce.number().default(5432),
  DB_USER: z.string(),
  DB_PASSWORD: z.string(),
  DB_NAME: z.string(),
  //   REDIS_HOST: z.string(),
  //   REDIS_PORT: z.coerce.number().default(6379),
  //   REDIS_USER: z.string().optional(),
  //   REDIS_PASSWORD: z.string().optional(),
  //   REDIS_SSL_ENABLED: z.boolean().default(false),
});

export type ConfigSchema = z.infer<typeof configSchema>;

export default configSchema;
