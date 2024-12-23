import { CanActivate, ExecutionContext, Injectable } from "@nestjs/common";
import { Reflector } from "@nestjs/core";
import { User } from "src/users/entities/user.entity";

import {
  AppAbility,
  CaslAbilityFactory,
} from "../casl-ability.factory/casl-ability.factory";
import { CHECK_POLICIES_KEY } from "./policies.decorator";
import { PolicyHandler } from "./policy.handler";

@Injectable()
export class PoliciesGuard implements CanActivate {
  constructor(
    private reflector: Reflector,
    private caslAbilityFactory: CaslAbilityFactory,
  ) {}

  async canActivate(context: ExecutionContext): Promise<boolean> {
    const policyHandlers =
      this.reflector.get<PolicyHandler[]>(
        CHECK_POLICIES_KEY,
        context.getHandler(),
      ) || [];

    const { user } = context.switchToHttp().getRequest<{ user: User }>();
    const ability = this.caslAbilityFactory.createForUser(user);

    console.log(user);
    console.log(ability.rules[1].conditions);

    return policyHandlers.every((handler) =>
      this.execPolicyHandler(handler, ability),
    );
  }

  private execPolicyHandler(handler: PolicyHandler, ability: AppAbility) {
    if (typeof handler === "function") {
      return handler(ability);
    }
    return handler.handle(ability);
  }
}
