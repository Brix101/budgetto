import { Cascade, Entity, ManyToOne, Property } from "@mikro-orm/core";

import { Category } from "../../categories/entities/category.entity";
import { BaseEntity } from "../../common/entities/base.entity";
import { User } from "../../users/entities/user.entity";

@Entity()
export class Transaction extends BaseEntity {
  @Property()
  amount!: number;

  @Property()
  description!: string;

  @Property()
  date = new Date();

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
