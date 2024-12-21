import { MikroOrmModule } from "@mikro-orm/nestjs";
import { Module } from "@nestjs/common";
import { CaslModule } from "src/casl/casl.module";
import { UsersModule } from "src/users/users.module";

import { CategoriesController } from "./categories.controller";
import { CategoriesService } from "./categories.service";
import { Category } from "./entities/category.entity";

@Module({
  imports: [MikroOrmModule.forFeature([Category]), CaslModule, UsersModule],
  controllers: [CategoriesController],
  providers: [CategoriesService],
})
export class CategoriesModule {}
