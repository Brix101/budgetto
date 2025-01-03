import {
  Body,
  Controller,
  Get,
  HttpCode,
  HttpStatus,
  Post,
  Request,
  UseGuards,
  UsePipes,
} from "@nestjs/common";
import { ZodValidationPipe } from "src/common/zod-validation.pipe";
import { User, UserDto } from "src/users/entities/user.entity";
import { UsersService } from "src/users/users.service";

import type { RefreshDto } from "@budgetto/schema/auth";
import type { CreateUserDto } from "@budgetto/schema/user";
import { refreshSchema } from "@budgetto/schema/auth";
import { createUserSchema } from "@budgetto/schema/user";

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
  @HttpCode(HttpStatus.OK)
  signIn(@Request() req: { user: User }) {
    return this.authService.signIn(req.user);
  }

  @Public()
  @Post("sign-up")
  @UsePipes(new ZodValidationPipe(createUserSchema))
  create(@Body() createUserDto: CreateUserDto) {
    return this.usersService.create(createUserDto);
  }

  @Public()
  @Post("refresh")
  @UsePipes(new ZodValidationPipe(refreshSchema))
  refresh(@Body() refreshDto: RefreshDto) {
    return this.authService.refresh(refreshDto);
  }

  @Get("profile")
  getProfile(@Request() req: { user: UserDto }) {
    return req.user;
  }
}
