import type { TestingModule } from "@nestjs/testing";
import { Test } from "@nestjs/testing";

import { BudgetsService } from "./budgets.service";

describe("BudgetsService", () => {
  let service: BudgetsService;

  beforeEach(async () => {
    const module: TestingModule = await Test.createTestingModule({
      providers: [BudgetsService],
    }).compile();

    service = module.get<BudgetsService>(BudgetsService);
  });

  it("should be defined", () => {
    expect(service).toBeDefined();
  });
});
