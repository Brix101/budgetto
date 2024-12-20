import { Injectable } from "@nestjs/common";

import { CreateBudgetDto, UpdateBudgetDto } from "@budgetto/schema";

@Injectable()
export class BudgetsService {
  create(createBudgetDto: CreateBudgetDto) {
    return "This action adds a new budget";
  }

  findAll() {
    return `This action returns all budget`;
  }

  findOne(id: number) {
    return `This action returns a #${id} budget`;
  }

  update(id: number, updateBudgetDto: UpdateBudgetDto) {
    return `This action updates a #${id} budget`;
  }

  remove(id: number) {
    return `This action removes a #${id} budget`;
  }
}
