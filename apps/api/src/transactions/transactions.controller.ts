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

import type {
  CreateTransactionDto,
  UpdateTransactionDto,
} from "@budgetto/schema";
import {
  createTransactionSchema,
  updateTransactionSchema,
} from "@budgetto/schema";

import { TransactionsService } from "./transactions.service";

@Controller("transactions")
export class TransactionsController {
  constructor(private readonly transactionsService: TransactionsService) {}

  @Post()
  @UsePipes(new ZodValidationPipe(createTransactionSchema))
  create(@Body() createTransactionDto: CreateTransactionDto) {
    return this.transactionsService.create(createTransactionDto);
  }

  @Get()
  findAll() {
    return this.transactionsService.findAll();
  }

  @Get(":id")
  findOne(@Param("id") id: string) {
    return this.transactionsService.findOne(+id);
  }

  @Patch(":id")
  @UsePipes(new ZodValidationPipe(updateTransactionSchema))
  update(
    @Param("id") id: string,
    @Body() updateTransactionDto: UpdateTransactionDto,
  ) {
    return this.transactionsService.update(+id, updateTransactionDto);
  }

  @Delete(":id")
  remove(@Param("id") id: string) {
    return this.transactionsService.remove(+id);
  }
}
