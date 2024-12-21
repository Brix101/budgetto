import { EntityDTO } from "@mikro-orm/core";
import {
  Body,
  Controller,
  Get,
  Post,
  Request,
  UseGuards,
  UsePipes,
} from "@nestjs/common";
import { ZodValidationPipe } from "src/common/zod-validation.pipe";
import { User } from "src/users/entities/user.entity";
import { UsersService } from "src/users/users.service";

import type { CreateUserDto } from "@budgetto/schema";
import { createUserSchema } from "@budgetto/schema";

import { Public } from "./auth.decorator";
import { AuthService } from "./auth.service";
import { LocalAuthGuard } from "./local-auth.guard";

@Controller("auth")
export class AuthController {
  constructor(
    private readonly authService: AuthService,
    private readonly usersService: UsersService,
  ) {}

  @Public()
  @UseGuards(LocalAuthGuard)
  @Post("sign-in")
  signIn(@Request() req: { user: User }) {
    return this.authService.signIn(req.user);
  }

  @Public()
  @Post()
  @UsePipes(new ZodValidationPipe(createUserSchema))
  create(@Body() createUserDto: CreateUserDto) {
    return this.usersService.create(createUserDto);
  }

  @Get("profile")
  getProfile(@Request() req: { user: EntityDTO<User> }) {
    return req.user;
  }

  @Post("/sign-out")
  async signOut(@Request() req) {
    return req.logout();
  }
}
