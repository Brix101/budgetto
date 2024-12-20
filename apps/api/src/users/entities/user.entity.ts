import type { Ref } from "@mikro-orm/core";
import { Entity, Property, Unique } from "@mikro-orm/core";

import { BaseEntity } from "../../common/entities/base.entity";

@Entity()
@Unique({ properties: ["email"] })
export class User extends BaseEntity {
  @Property()
  name!: string;

  @Property()
  email!: string;

  @Property({ lazy: true, ref: true, hidden: true })
  password!: Ref<string>;

  @Property({ default: false, hidden: true })
  isConfirmed = false;
}
