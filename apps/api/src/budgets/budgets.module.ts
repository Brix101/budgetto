import { MikroOrmModule } from "@mikro-orm/nestjs";
import { Module } from "@nestjs/common";
import { CaslModule } from "src/casl/casl.module";

import { BudgetsController } from "./budgets.controller";
import { BudgetsService } from "./budgets.service";
import { Budget } from "./entities/budget.entity";

@Module({
  imports: [MikroOrmModule.forFeature([Budget]), CaslModule],
  controllers: [BudgetsController],
  providers: [BudgetsService],
})
export class BudgetsModule {}
