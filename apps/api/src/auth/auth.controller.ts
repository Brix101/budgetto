import {
  Body,
  Controller,
  HttpCode,
  HttpStatus,
  Post,
  UsePipes,
} from "@nestjs/common";
import { ZodValidationPipe } from "src/common/zod-validation.pipe";

import type { SignInDto } from "./dto/sign-in.dto";
import { AuthService } from "./auth.service";
import { signInSchema } from "./dto/sign-in.dto";

@Controller("auth")
export class AuthController {
  constructor(private readonly authService: AuthService) {}

  @HttpCode(HttpStatus.OK)
  @Post("sign-in")
  @UsePipes(new ZodValidationPipe(signInSchema))
  signIn(@Body() signInDto: SignInDto) {
    return this.authService.signIn(signInDto.email, signInDto.password);
  }
}
