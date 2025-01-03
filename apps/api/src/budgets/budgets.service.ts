import {
  Injectable,
  InternalServerErrorException,
  Logger,
  NotFoundException,
} from "@nestjs/common";
import { UserDto } from "src/users/entities/user.entity";

import { CreateBudgetDto, UpdateBudgetDto } from "@budgetto/schema/budget";

import { BudgetRepository } from "./budgets.repository";

@Injectable()
export class BudgetsService {
  private logger = new Logger(BudgetsService.name);

  constructor(private readonly repo: BudgetRepository) {}

  async create(user: UserDto, { categoryId, ...rest }: CreateBudgetDto) {
    try {
      const budget = this.repo.create({
        ...rest,
        category: categoryId,
        user: user.id,
      });

      await this.repo.insert(budget);

      return budget;
    } catch (error) {
      this.logger.error(error);

      throw new InternalServerErrorException(error);
    }
  }

  async findAll(user: UserDto) {
    try {
      const budgets = await this.repo.findAll({ where: { user: user.id } });

      return budgets;
    } catch (error) {
      this.logger.error(error);

      throw new InternalServerErrorException(error);
    }
  }

  async findOne(id: number) {
    try {
      const budget = await this.repo.findOneOrFail({ id });

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
      const budget = await this.findOne(id);

      budget.assign(updateBudgetDto);

      await this.repo.nativeUpdate({ id }, updateBudgetDto);

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
      const budget = await this.findOne(id);

      await this.repo.nativeDelete({ id });

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
