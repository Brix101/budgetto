import { MikroOrmModule } from "@mikro-orm/nestjs";
import { Module } from "@nestjs/common";
import { ConfigModule } from "@nestjs/config";

import { AuthModule } from "./auth/auth.module";
import { BudgetsModule } from "./budgets/budgets.module";
import { CacheModule } from "./cache/cache.module";
import { CaslModule } from "./casl/casl.module";
import { CategoriesModule } from "./categories/categories.module";
import configSchema from "./config/config.schema";
import configuration from "./config/configuration";
import databaseConfig from "./config/database.config";
import jwtConfig from "./config/jwt.config";
import redisConfig from "./config/redis.config";
import mikroOrmConfig from "./mikro-orm.config";
import { TransactionsModule } from "./transactions/transactions.module";
import { UsersModule } from "./users/users.module";
import { UtilModule } from "./util/util.module";

@Module({
  imports: [
    ConfigModule.forRoot({
      envFilePath: [".env.development.local", ".env"],
      load: [configuration, databaseConfig, redisConfig, jwtConfig],
      validate: (config) => configSchema.parse(config),
      isGlobal: true,
    }),
    MikroOrmModule.forRoot(mikroOrmConfig),
    CacheModule,
    UsersModule,
    CategoriesModule,
    BudgetsModule,
    TransactionsModule,
    AuthModule,
    UtilModule,
    CaslModule,
  ],
})
export class AppModule {}
