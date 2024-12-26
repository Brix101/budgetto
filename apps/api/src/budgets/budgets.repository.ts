import { EntityRepository } from "@mikro-orm/core";

import type { Budget } from "./entities/budget.entity";

export class BudgetRepository extends EntityRepository<Budget> {}
