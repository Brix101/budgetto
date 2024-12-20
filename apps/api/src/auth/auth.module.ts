import { Module } from "@nestjs/common";
import { UsersModule } from "src/users/users.module";
import { UtilModule } from "src/util/util.module";

import { AuthController } from "./auth.controller";
import { AuthService } from "./auth.service";

@Module({
  imports: [UtilModule, UsersModule],
  providers: [AuthService],
  controllers: [AuthController],
})
export class AuthModule {}
