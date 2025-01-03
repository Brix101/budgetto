import {
  Injectable,
  InternalServerErrorException,
  Logger,
  NotFoundException,
} from "@nestjs/common";
import { UserDto } from "src/users/entities/user.entity";

import {
  CreateTransactionDto,
  UpdateTransactionDto,
} from "@budgetto/schema/transaction";

import { TransactionRepository } from "./transactions.repository";

@Injectable()
export class TransactionsService {
  private logger = new Logger(TransactionsService.name);

  constructor(private readonly repo: TransactionRepository) {}

  async create(user: UserDto, { categoryId, ...rest }: CreateTransactionDto) {
    try {
      const transaction = this.repo.create({
        ...rest,
        category: categoryId,
        user: user.id,
      });

      await this.repo.insert(transaction);

      return transaction;
    } catch (error) {
      this.logger.error(error);

      throw new InternalServerErrorException(error);
    }
  }

  async findAll(user: UserDto) {
    try {
      const transactions = await this.repo.findByCursor(
        {
          user: { id: user.id },
        },
        {
          first: 10,
          orderBy: { createdAt: "DESC" },
          populate: ["category"],
        },
      );
      return transactions;
    } catch (error) {
      this.logger.error(error);

      throw new InternalServerErrorException(error);
    }
  }

  async findOne(id: number) {
    try {
      const transcations = await this.repo.findOneOrFail({ id });

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
      budget.assign(updateTransactionDto);

      await this.repo.nativeUpdate({ id }, updateTransactionDto);

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

      await this.repo.nativeDelete({ id });

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
