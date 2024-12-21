import type { Reflector } from "@nestjs/core";

import type { CaslAbilityFactory } from "../casl-ability.factory/casl-ability.factory";
import { PoliciesGuard } from "./policies.guard";

describe("PoliciesGuard", () => {
  it("should be defined", () => {
    const reflector = {} as Reflector;
    const caslAbilityFactory = {} as CaslAbilityFactory;
    expect(new PoliciesGuard(reflector, caslAbilityFactory)).toBeDefined();
  });
});
