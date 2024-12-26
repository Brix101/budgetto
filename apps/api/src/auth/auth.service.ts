import { Injectable } from "@nestjs/common";
import { JwtService } from "@nestjs/jwt";
import { jwtConstants } from "src/auth/auth.constants";
import { CacheService } from "src/cache/cache.service";
import { User } from "src/users/entities/user.entity";
import { UsersService } from "src/users/users.service";
import { PasswordUtilService } from "src/util/password-util.service";

@Injectable()
export class AuthService {
  constructor(
    private readonly usersService: UsersService,
    private readonly passwordUtilService: PasswordUtilService,
    private jwtService: JwtService,
    private readonly cacheService: CacheService,
  ) {}

  async validateUser(email: string, pass: string) {
    try {
      const user = await this.usersService.findOne({ email });

      const hashedPass = await user.password.load();

      const isVerified = await this.passwordUtilService.verify(
        hashedPass,
        pass,
      );

      if (user && isVerified) {
        return user;
      }

      return null;
    } catch {
      return null;
    }
  }

  async signIn(user: User): Promise<any> {
    const payload = user.toPayload();

    await this.cacheService.set(
      jwtConstants.accessPrefix + payload.sub,
      user.toObject(),
      jwtConstants.accessTokenExpires,
    );

    return {
      access_token: await this.jwtService.signAsync(payload),
    };
  }
}
