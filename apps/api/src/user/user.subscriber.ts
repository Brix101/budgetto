import { EntityName, EventArgs, EventSubscriber } from "@mikro-orm/core";
import { EntityManager } from "@mikro-orm/postgresql";
import { Injectable } from "@nestjs/common";
import { PasswordUtilService } from "src/util/password-util.service";

import { User } from "./entities/user.entity";

@Injectable()
export class UserSubscriber implements EventSubscriber<User> {
  constructor(
    private readonly em: EntityManager,
    private readonly passwordUtilsService: PasswordUtilService,
  ) {
    this.em.getEventManager().registerSubscriber(this);
  }

  getSubscribedEntities(): EntityName<User>[] {
    return [User];
  }

  async beforeCreate(args: EventArgs<User>) {
    const user = args.entity;
    const password = await user.password.load();
    const hashedPassword = await this.passwordUtilsService.hash(password);

    user.password.set(hashedPassword);
  }
}
