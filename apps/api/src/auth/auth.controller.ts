import { Controller, Get, Post, Request, UseGuards } from "@nestjs/common";
import { User } from "src/users/entities/user.entity";

import { Public } from "./auth.decorator";
import { AuthService } from "./auth.service";
import { LocalAuthGuard } from "./local-auth.guard";

@Controller("auth")
export class AuthController {
  constructor(private readonly authService: AuthService) {}

  @Public()
  @UseGuards(LocalAuthGuard)
  @Post("sign-in")
  signIn(@Request() req: { user: Partial<User> }) {
    return this.authService.signIn(req.user);
  }

  @Get("profile")
  getProfile(@Request() req) {
    return req.user;
  }

  @Post("/sign-out")
  async signOut(@Request() req) {
    return req.logout();
  }
}
