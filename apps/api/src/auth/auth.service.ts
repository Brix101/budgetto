import { Injectable, Logger } from "@nestjs/common";
import { User } from "src/users/entities/user.entity";
import { UsersService } from "src/users/users.service";
import { JwtUtilService } from "src/util/jwt-util/jwt-util.service";
import { PasswordUtilService } from "src/util/password-util/password-util.service";

import { RefreshDto, SignInResponseDto } from "@budgetto/schema";

@Injectable()
export class AuthService {
  private readonly logger = new Logger(AuthService.name);

  constructor(
    private readonly usersService: UsersService,
    private readonly passwordUtilService: PasswordUtilService,
    private readonly jwtUtilService: JwtUtilService,
  ) {}

  async validateUser(email: string, pass: string) {
    try {
      const user = await this.usersService.findOne({ email });

      this.logger.log(user);

      const hashedPass = await user.password.load();

      this.logger.log(hashedPass);

      const isVerified = await this.passwordUtilService.verify(
        hashedPass,
        pass,
      );

      this.logger.log({ isVerified });

      if (user && isVerified) {
        return user;
      }

      return null;
    } catch {
      return null;
    }
  }

  async signIn(user: User): Promise<SignInResponseDto> {
    this.logger.log(
      "++++++++++++++++++++++++++++++++++++++++++++++++++++++++++",
    );
    this.logger.log(user.toObject());

    const tokenPair = await this.jwtUtilService.getTokenPair(user);

    this.logger.log(tokenPair);
    return tokenPair;
  }

  async refresh({ refreshToken }: RefreshDto): Promise<SignInResponseDto> {
    return this.jwtUtilService.refreshAccessToken(refreshToken);
  }
}
