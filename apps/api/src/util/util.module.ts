import { Module } from "@nestjs/common";

import { PasswordUtilService } from "./password-util.service";

@Module({
  providers: [PasswordUtilService],
  exports: [PasswordUtilService],
})
export class UtilModule {}
