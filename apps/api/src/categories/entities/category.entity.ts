import {
  Cascade,
  Entity,
  Enum,
  ManyToOne,
  Property,
  Unique,
} from "@mikro-orm/core";

import { CategoryType } from "@budgetto/schema";

import { BaseEntity } from "../../common/entities/base.entity";
import { User } from "../../users/entities/user.entity";

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
    hidden: true,
  })
  user?: User;
}
