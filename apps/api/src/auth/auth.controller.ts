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
import { User, UserDto } from "src/users/entities/user.entity";
import { UsersService } from "src/users/users.service";

import type { CreateUserDto, RefreshDto } from "@budgetto/schema";
import { createUserSchema, refreshSchema } from "@budgetto/schema";

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
  @Post("sign-up")
  @UsePipes(new ZodValidationPipe(createUserSchema))
  create(@Body() createUserDto: CreateUserDto) {
    return this.usersService.create(createUserDto);
  }

  @Public()
  @Post("refresh")
  // @UseGuards(AuthGuard(["refresh"]))
  @UsePipes(new ZodValidationPipe(refreshSchema))
  refresh(@Body() refreshDto: RefreshDto) {
    return this.authService.refresh(refreshDto);
  }

  @Get("profile")
  getProfile(@Request() req: { user: UserDto }) {
    return req.user;
  }
}
