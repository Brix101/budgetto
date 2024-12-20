import {
  Body,
  Controller,
  Delete,
  Get,
  Param,
  Patch,
  Post,
  UsePipes,
} from "@nestjs/common";
import { ZodValidationPipe } from "src/common/zod-validation.pipe";

import type { CreateBudgetDto, UpdateBudgetDto } from "@budgetto/schema";
import { createBudgetSchema, updateBudgetSchema } from "@budgetto/schema";

import { BudgetsService } from "./budgets.service";

@Controller("budgets")
export class BudgetsController {
  constructor(private readonly budgetsService: BudgetsService) {}

  @Post()
  @UsePipes(new ZodValidationPipe(createBudgetSchema))
  create(@Body() createBudgetDto: CreateBudgetDto) {
    return this.budgetsService.create(createBudgetDto);
  }

  @Get()
  findAll() {
    return this.budgetsService.findAll();
  }

  @Get(":id")
  findOne(@Param("id") id: string) {
    return this.budgetsService.findOne(+id);
  }

  @Patch(":id")
  @UsePipes(new ZodValidationPipe(updateBudgetSchema))
  update(@Param("id") id: string, @Body() updateBudgetDto: UpdateBudgetDto) {
    return this.budgetsService.update(+id, updateBudgetDto);
  }

  @Delete(":id")
  remove(@Param("id") id: string) {
    return this.budgetsService.remove(+id);
  }
}
