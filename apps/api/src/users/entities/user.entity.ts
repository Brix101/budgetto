import type { EntityDTO, Ref } from "@mikro-orm/core";
import {
  Entity,
  EntityRepositoryType,
  Property,
  Unique,
} from "@mikro-orm/core";
import { v4 as uuidv4 } from "uuid";

import { BaseEntity } from "../../common/entities/base.entity";
import { UserRepository } from "../users.repository";

@Entity({ repository: () => UserRepository })
@Unique({ properties: ["email"] })
export class User extends BaseEntity {
  [EntityRepositoryType]?: UserRepository;

  @Property()
  name!: string;

  @Property()
  email!: string;

  @Property({ lazy: true, ref: true, hidden: true })
  password!: Ref<string>;

  @Property({ default: false, hidden: true })
  isConfirmed = false;

  @Property({ default: false, hidden: true })
  isAdmin = false;

  public generateUUID(): string {
    return uuidv4();
  }
}

export type UserDto = EntityDTO<User>;
