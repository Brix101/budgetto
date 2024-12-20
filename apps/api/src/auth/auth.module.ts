import { Module } from "@nestjs/common";
import { UserModule } from "src/user/user.module";
import { UtilModule } from "src/util/util.module";

import { AuthController } from "./auth.controller";
import { AuthService } from "./auth.service";

@Module({
  imports: [UtilModule, UserModule],
  providers: [AuthService],
  controllers: [AuthController],
})
export class AuthModule {}
