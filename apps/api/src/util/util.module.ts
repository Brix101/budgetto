import { Module } from "@nestjs/common";
import { ConfigModule } from "@nestjs/config";
import { JwtModule } from "@nestjs/jwt";
import { CacheModule } from "src/cache/cache.module";

import { JwtUtilService } from "./jwt-util/jwt-util.service";
import { PasswordUtilService } from "./password-util/password-util.service";

@Module({
  imports: [ConfigModule, JwtModule, CacheModule],
  providers: [PasswordUtilService, JwtUtilService],
  exports: [PasswordUtilService, JwtUtilService],
})
export class UtilModule {}
