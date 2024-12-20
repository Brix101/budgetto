import type { TestingModule } from "@nestjs/testing";
import { Test } from "@nestjs/testing";

import { PasswordUtilService } from "./password-util.service";

describe("UtilsService", () => {
  let service: PasswordUtilService;

  beforeEach(async () => {
    const module: TestingModule = await Test.createTestingModule({
      providers: [PasswordUtilService],
    }).compile();

    service = module.get<PasswordUtilService>(PasswordUtilService);
  });

  it("should be defined", () => {
    expect(service).toBeDefined();
  });
});
