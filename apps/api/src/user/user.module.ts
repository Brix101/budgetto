import { MikroOrmModule } from "@mikro-orm/nestjs";
import { Module } from "@nestjs/common";
import { UtilModule } from "src/util/util.module";

import { User } from "./entities/user.entity";
import { UserController } from "./user.controller";
import { UserService } from "./user.service";
import { UserSubscriber } from "./user.subscriber";

@Module({
  imports: [MikroOrmModule.forFeature([User]), UtilModule],
  controllers: [UserController],
  providers: [UserService, UserSubscriber],
  exports: [UserService],
})
export class UserModule {}
