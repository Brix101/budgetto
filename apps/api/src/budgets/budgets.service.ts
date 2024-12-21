import { EntityManager, MikroORM } from "@mikro-orm/core";
import {
  Injectable,
  InternalServerErrorException,
  Logger,
  NotFoundException,
} from "@nestjs/common";
import { UserDto } from "src/users/entities/user.entity";

import { CreateBudgetDto, UpdateBudgetDto } from "@budgetto/schema";

import { Budget } from "./entities/budget.entity";

@Injectable()
export class BudgetsService {
  private logger = new Logger(BudgetsService.name);

  constructor(
    private readonly orm: MikroORM,
    private readonly em: EntityManager,
  ) {}

  async create(user: UserDto, createBudgetDto: CreateBudgetDto) {
    try {
      const budget = this.em.create(Budget, {
        ...createBudgetDto,
        category: createBudgetDto.categoryId,
        user: user.id,
      });

      await this.em.persistAndFlush(budget);

      return budget;
    } catch (error) {
      this.logger.error(error);

      throw new InternalServerErrorException(error);
    }
  }

  async findAll(user: UserDto) {
    try {
      const budgets = await this.em.find(Budget, {
        user: { id: user.id },
      });
      return budgets;
    } catch (error) {
      this.logger.error(error);

      throw new InternalServerErrorException(error);
    }
  }

  async findOne(id: number) {
    try {
      const budget = await this.em.findOneOrFail(Budget, { id });

      return budget;
    } catch (error) {
      this.logger.error(error);
      throw new NotFoundException([
        {
          validation: "id",
          message: "Budget not found",
          path: ["id"],
        },
      ]);
    }
  }

  async update(id: number, updateBudgetDto: UpdateBudgetDto) {
    try {
      const budget = this.findOne(id);
      this.em.assign(budget, updateBudgetDto);

      await this.em.nativeUpdate(Budget, { id }, updateBudgetDto);

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
      const budget = this.findOne(id);

      await this.em.nativeDelete(Budget, { id });

      return budget;
    } catch (error) {
      if (error instanceof NotFoundException) {
        throw error;
      }
      this.logger.error(error);
      throw new InternalServerErrorException(error);
    }
  }
}
