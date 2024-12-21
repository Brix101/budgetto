import {
  AbilityBuilder,
  createMongoAbility,
  ExtractSubjectType,
  InferSubjects,
  MongoAbility,
  MongoQuery,
} from "@casl/ability";
import { Injectable } from "@nestjs/common";
import { Budget } from "src/budgets/entities/budget.entity";
import { Category } from "src/categories/entities/category.entity";
import { User } from "src/users/entities/user.entity";

import Action from "../casl-action.enum";

type Subjects =
  | InferSubjects<typeof Budget | typeof Category | typeof User>
  | "all";
type PossibleAbilities = [Action, Subjects];
type Conditions = MongoQuery;

export type AppAbility = MongoAbility<PossibleAbilities, Conditions>;

@Injectable()
export class CaslAbilityFactory {
  createForUser(user: User) {
    const { can, build } = new AbilityBuilder(
      createMongoAbility<PossibleAbilities, Conditions>,
    );

    if (user.isAdmin) {
      can(Action.Manage, "all"); // read-write access to everything
    }

    // Users
    // can(Action.Manage, User, { id: user.id });
    can(Action.Manage, Budget, { user: user });
    can(Action.Manage, Category, { user: user });

    return build({
      // Read https://casl.js.org/v6/en/guide/subject-type-detection#use-classes-as-subject-types for details
      detectSubjectType: (item) =>
        item.constructor as ExtractSubjectType<Subjects>,
    });
  }
}
