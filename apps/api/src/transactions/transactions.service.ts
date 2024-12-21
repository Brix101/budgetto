import { EntityManager, MikroORM } from "@mikro-orm/core";
import {
  Injectable,
  InternalServerErrorException,
  Logger,
  NotFoundException,
} from "@nestjs/common";
import { UserDto } from "src/users/entities/user.entity";

import { CreateTransactionDto, UpdateTransactionDto } from "@budgetto/schema";

import { Transaction } from "./entities/transaction.entity";

@Injectable()
export class TransactionsService {
  private logger = new Logger(TransactionsService.name);

  constructor(
    private readonly orm: MikroORM,
    private readonly em: EntityManager,
  ) {}

  async create(user: UserDto, createTransactionDto: CreateTransactionDto) {
    try {
      const { categoryId, ...rest } = createTransactionDto;
      const transaction = this.em.create(Transaction, {
        ...rest,
        category: categoryId,
        user: user.id,
      });

      await this.em.persistAndFlush(transaction);

      return transaction;
    } catch (error) {
      this.logger.error(error);

      throw new InternalServerErrorException(error);
    }
  }

  async findAll(user: UserDto) {
    try {
      const transactions = await this.em.find(Transaction, {
        user: { id: user.id },
      });
      return transactions;
    } catch (error) {
      this.logger.error(error);

      throw new InternalServerErrorException(error);
    }
  }

  async findOne(id: number) {
    try {
      const transcations = await this.em.findOneOrFail(Transaction, { id });

      return transcations;
    } catch (error) {
      this.logger.error(error);
      throw new NotFoundException([
        {
          validation: "id",
          message: "Transaction not found",
          path: ["id"],
        },
      ]);
    }
  }

  async update(id: number, updateTransactionDto: UpdateTransactionDto) {
    try {
      const budget = await this.findOne(id);
      this.em.assign(budget, updateTransactionDto);

      await this.em.nativeUpdate(Transaction, { id }, updateTransactionDto);

      return budget;
    } catch (error) {
      if (error instanceof NotFoundException) {
        throw error;
      }
      this.logger.error(error);
      throw new InternalServerErrorException(error);
    }
  }

  async remove(id: number) {
    try {
      const transaction = await this.findOne(id);

      await this.em.nativeDelete(Transaction, { id });

      return transaction;
    } catch (error) {
      if (error instanceof NotFoundException) {
        throw error;
      }
      this.logger.error(error);
      throw new InternalServerErrorException(error);
    }
  }
}
