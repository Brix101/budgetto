import {
  Body,
  Controller,
  Delete,
  Get,
  Param,
  Patch,
  Post,
  Request,
  UsePipes,
} from "@nestjs/common";
import { ZodValidationPipe } from "src/common/zod-validation.pipe";
import { UserDto } from "src/users/entities/user.entity";

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
  create(
    @Body() createTransactionDto: CreateTransactionDto,
    @Request() req: { user: UserDto },
  ) {
    return this.transactionsService.create(req.user, createTransactionDto);
  }

  @Get()
  findAll(@Request() req: { user: UserDto }) {
    return this.transactionsService.findAll(req.user);
  }

  @Get(":id")
  findOne(@Param("id") id: string, @Request() _req: { user: UserDto }) {
    return this.transactionsService.findOne(+id);
  }

  @Patch(":id")
  @UsePipes(new ZodValidationPipe(updateTransactionSchema))
  update(
    @Param("id") id: string,
    @Body() updateTransactionDto: UpdateTransactionDto,
    @Request() _req: { user: UserDto },
  ) {
    return this.transactionsService.update(+id, updateTransactionDto);
  }

  @Delete(":id")
  remove(@Param("id") id: string, @Request() _req: { user: UserDto }) {
    return this.transactionsService.remove(+id);
  }
}
