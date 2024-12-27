import KeyvRedis from "@keyv/redis";
import { Module } from "@nestjs/common";
import { ConfigModule, ConfigService } from "@nestjs/config";
import { Cacheable } from "cacheable";
import { RedisConfig } from "src/config/redis.config";

import { CacheService } from "./cache.service";

@Module({
  imports: [ConfigModule],
  providers: [
    {
      provide: "CACHE_INSTANCE",
      useFactory: (configService: ConfigService) => {
        const redisConfig = configService.get<RedisConfig>("redis");

        const secondary = new KeyvRedis(
          `rediss://${redisConfig.username}:${redisConfig.password}@${redisConfig.host}:${redisConfig.port}`,
        );

        return new Cacheable({ secondary });
      },
      inject: [ConfigService],
    },
    CacheService,
  ],
  exports: [CacheService],
})
export class CacheModule {}
