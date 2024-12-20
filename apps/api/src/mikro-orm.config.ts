import { defineConfig } from "@mikro-orm/core";
import { PostgreSqlDriver } from "@mikro-orm/postgresql";
import { SqlHighlighter } from "@mikro-orm/sql-highlighter";
import { Logger } from "@nestjs/common";
import * as dotenv from "dotenv";

import { Budget } from "./budget/entities/budget.entity";
import { Category } from "./category/entities/category.entity";
import { BaseEntity } from "./common/entities/base.entity";
import { Transaction } from "./transaction/entities/transaction.entity";
import { User } from "./user/entities/user.entity";

dotenv.config({
  path: [".env", ".env.development.local"],
});

const logger = new Logger("MikroORM-Config");

export default defineConfig({
  host: process.env.DB_HOST,
  port: parseInt(process.env.DB_PORT ?? "5432", 10),
  user: process.env.DB_USER,
  password: process.env.DB_PASSWORD,
  dbName: process.env.DB_NAME,
  entities: [User, Category, Budget, Transaction, BaseEntity],
  debug: process.env.NODE_ENV !== "production",
  driver: PostgreSqlDriver,
  highlighter: new SqlHighlighter(),
  logger: logger.log.bind(logger),
});
