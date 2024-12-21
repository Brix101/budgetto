import { z } from "zod";

export const serverConfigSchema = z.object({
  NODE_ENV: z
    .enum(["development", "production", "test", "provision"])
    .default("development"),
  PORT: z.coerce.number().default(5000),
});

export const dbConfigSchema = z.object({
  DB_HOST: z.string(),
  DB_PORT: z.coerce.number().default(5432),
  DB_USER: z.string(),
  DB_PASSWORD: z.string(),
  DB_NAME: z.string().default("budgetto"),
});

export const redisConfigSchema = z.object({
  REDIS_HOST: z.string(),
  REDIS_PORT: z.coerce.number().default(6379),
  REDIS_USER: z.string().optional(),
  REDIS_PASSWORD: z.string().optional(),
  REDIS_SSL_ENABLED: z.boolean().default(false),
});

const configSchema = serverConfigSchema
  .merge(dbConfigSchema)
  .merge(redisConfigSchema);

export type DbConfig = z.infer<typeof dbConfigSchema>;
export type RedisConfig = z.infer<typeof redisConfigSchema>;
export type ServerConfig = z.infer<typeof serverConfigSchema>;

export default configSchema;
