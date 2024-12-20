import { MikroOrmModule } from "@mikro-orm/nestjs";
import { Module } from "@nestjs/common";
import { ConfigModule } from "@nestjs/config";

import { BudgetModule } from "./budget/budget.module";
import { CategoryModule } from "./category/category.module";
import configSchema from "./config/config.schema";
import configuration from "./config/configuration";
import databaseConfig from "./config/database.config";
import mikroOrmConfig from "./mikro-orm.config";
import { TransactionModule } from "./transaction/transaction.module";
import { UserModule } from "./user/user.module";

@Module({
  imports: [
    ConfigModule.forRoot({
      envFilePath: [".env.development.local", ".env"],
      load: [configuration, databaseConfig],
      validate: (config) => configSchema.parse(config),
      isGlobal: true,
    }),
    MikroOrmModule.forRoot(mikroOrmConfig),
    UserModule,
    CategoryModule,
    BudgetModule,
    TransactionModule,
  ],
  controllers: [],
  providers: [],
})
export class AppModule {}
