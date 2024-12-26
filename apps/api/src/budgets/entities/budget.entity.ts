import {
  Cascade,
  Entity,
  EntityRepositoryType,
  ManyToOne,
  Property,
} from "@mikro-orm/core";

import { Category } from "../../categories/entities/category.entity";
import { BaseEntity } from "../../common/entities/base.entity";
import { User } from "../../users/entities/user.entity";
import { BudgetRepository } from "../budgets.repository";

@Entity({ repository: () => BudgetRepository })
export class Budget extends BaseEntity {
  [EntityRepositoryType]?: BudgetRepository;

  @Property()
  amount!: number;

  @Property()
  description!: string;

  @Property()
  startDate = new Date();

  @Property()
  endDate = new Date();

  @ManyToOne(() => Category, {
    cascade: [Cascade.PERSIST, Cascade.REMOVE],
    nullable: true,
  })
  category!: Category;

  @ManyToOne(() => User, {
    cascade: [Cascade.PERSIST, Cascade.REMOVE],
    nullable: true,
  })
  user!: User;
}
