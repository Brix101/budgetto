import {
  Cascade,
  Entity,
  Enum,
  ManyToOne,
  Property,
  Unique,
} from "@mikro-orm/core";

import { BaseEntity } from "../../common/entities/base.entity";
import { User } from "../../users/entities/user.entity";

export enum CategoryType {
  INCOME = "income",
  EXPENSE = "expense",
}

@Entity()
@Unique({ properties: ["name", "user"] })
export class Category extends BaseEntity {
  @Property()
  name!: string;

  @Property({ nullable: true })
  description!: string;

  @Enum(() => CategoryType)
  type!: CategoryType;

  @ManyToOne(() => User, {
    cascade: [Cascade.PERSIST, Cascade.REMOVE],
    nullable: true,
  })
  user?: User;
}
