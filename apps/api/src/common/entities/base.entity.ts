import {
  BaseEntity as MikroEntity,
  Opt,
  PrimaryKey,
  Property,
} from "@mikro-orm/core";

export abstract class BaseEntity extends MikroEntity {
  @PrimaryKey()
  id!: number;

  @Property()
  createdAt: Date & Opt = new Date();

  @Property({ onUpdate: () => new Date() })
  updatedAt: Date & Opt = new Date();
}
