import {
  Cascade,
  Entity,
  EntityRepositoryType,
  Enum,
  ManyToOne,
  Property,
  Unique,
} from "@mikro-orm/core";
import { UserRepository } from "src/users/users.repository";

import { CategoryType } from "@budgetto/schema";

import { BaseEntity } from "../../common/entities/base.entity";
import { User } from "../../users/entities/user.entity";
import { CategoryRepository } from "../categories.repository";

@Entity({ repository: () => CategoryRepository })
@Unique({ properties: ["name", "user"] })
export class Category extends BaseEntity {
  [EntityRepositoryType]?: UserRepository;

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
