import {
  Body,
  Controller,
  Delete,
  Get,
  Param,
  Patch,
  Post,
  Request,
  UseGuards,
  UsePipes,
} from "@nestjs/common";
import { AppAbility } from "src/casl/casl-ability.factory/casl-ability.factory";
import Action from "src/casl/casl-action.enum";
import { CheckPolicies } from "src/casl/policies/policies.decorator";
import { PoliciesGuard } from "src/casl/policies/policies.guard";
import { ZodValidationPipe } from "src/common/zod-validation.pipe";
import { UserDto } from "src/users/entities/user.entity";

import type { CreateBudgetDto, UpdateBudgetDto } from "@budgetto/schema";
import { createBudgetSchema, updateBudgetSchema } from "@budgetto/schema";

import { BudgetsService } from "./budgets.service";
import { Budget } from "./entities/budget.entity";

@Controller("budgets")
export class BudgetsController {
  constructor(private readonly budgetsService: BudgetsService) {}

  @Post()
  @UsePipes(new ZodValidationPipe(createBudgetSchema))
  create(
    @Body() createBudgetDto: CreateBudgetDto,
    @Request() req: { user: UserDto },
  ) {
    return this.budgetsService.create(req.user, createBudgetDto);
  }

  @Get()
  @UseGuards(PoliciesGuard)
  @CheckPolicies((ability: AppAbility) => ability.can(Action.Read, Budget))
  findAll(@Request() req: { user: UserDto }) {
    return this.budgetsService.findAll(req.user);
  }

  @Get(":id")
  findOne(@Param("id") id: string, @Request() _req: { user: UserDto }) {
    return this.budgetsService.findOne(+id);
  }

  @Patch(":id")
  @UsePipes(new ZodValidationPipe(updateBudgetSchema))
  update(
    @Param("id") id: string,
    @Body() updateBudgetDto: UpdateBudgetDto,
    @Request() _req: { user: UserDto },
  ) {
    return this.budgetsService.update(+id, updateBudgetDto);
  }

  @Delete(":id")
  remove(@Param("id") id: string, @Request() _req: { user: UserDto }) {
    return this.budgetsService.remove(+id);
  }
}
