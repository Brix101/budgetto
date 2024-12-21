import type { EntityDTO, Ref } from "@mikro-orm/core";
import { Entity, Property, Unique } from "@mikro-orm/core";
import { v4 as uuidv4 } from "uuid";

import { BaseEntity } from "../../common/entities/base.entity";
import { UserPayloadDto } from "../dto/user-payload.dto";

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

  @Property({ default: false, hidden: true })
  isAdmin = false;

  public toPayload(): UserPayloadDto {
    return {
      sub: uuidv4(),
      name: this.name,
      email: this.email,
    };
  }
}

export type UserDto = EntityDTO<User>;
