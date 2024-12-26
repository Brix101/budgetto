import { Injectable, Logger, UnauthorizedException } from "@nestjs/common";
import { ConfigService } from "@nestjs/config";
import { JwtService } from "@nestjs/jwt";
import { jwtConstants } from "src/auth/auth.constants";
import { CacheService } from "src/cache/cache.service";
import { JwtConfig } from "src/config/jwt.config";
import { User, UserDto } from "src/users/entities/user.entity";
import { UsersService } from "src/users/users.service";
import { PasswordUtilService } from "src/util/password-util.service";

import { RefreshDto, SignInResponseDto } from "@budgetto/schema";

@Injectable()
export class AuthService {
  private readonly logger = new Logger(AuthService.name);

  constructor(
    private readonly usersService: UsersService,
    private readonly passwordUtilService: PasswordUtilService,
    private jwtService: JwtService,
    private readonly cacheService: CacheService,
    private readonly configService: ConfigService,
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

  async signIn(user: User): Promise<SignInResponseDto> {
    const payload = user.toPayload();

    const jwtConfig: JwtConfig = this.configService.get<JwtConfig>("jwt");

    const accessToken = await this.genAcessToken(payload.sub, user.toObject());
    const refreshToken = await this.jwtService.signAsync(
      { sub: payload.sub },
      {
        privateKey: this.configService.get<string>("jwt.refreshPrivateKey"),
        expiresIn: this.configService.get<number>("jwt.refreshExpiresIn"),
        algorithm: "RS256",
      },
    );

    return {
      accessToken,
      refreshToken,
      expiresIn: jwtConstants.accessExpiresIn,
    };
  }

  async refresh({ refreshToken }: RefreshDto): Promise<SignInResponseDto> {
    try {
      const { sub } = await this.jwtService.verifyAsync<{ sub: string }>(
        refreshToken,
        {
          publicKey: this.configService.get<string>("jwt.refreshPublicKey"),
          algorithms: ["RS256"],
        },
      );

      const payload = await this.cacheService.get<UserDto>(
        jwtConstants.keyPrefix + sub,
      );

      if (!payload) {
        throw new UnauthorizedException();
      }

      const accessToken = await this.genAcessToken(sub, payload);

      return {
        accessToken,
        refreshToken,
        expiresIn: jwtConstants.accessExpiresIn,
      };
    } catch (e) {
      console.error(e);
      throw new UnauthorizedException([
        {
          code: "invalid_token",
          message: "Invalid refresh token",
          path: ["refreshToken"],
        },
      ]);
    }
  }

  private async genAcessToken(sub: string, payload: UserDto) {
    const user = await this.cacheService.set(
      jwtConstants.keyPrefix + sub,
      payload,
      jwtConstants.accessExpiresIn * 1000,
    );

    const accessToken = await this.jwtService.signAsync(
      {
        sub,
        email: user.email,
        name: user.name,
      },
      {
        privateKey: this.configService.get<string>("jwt.accessPrivateKey"),
        expiresIn: this.configService.get<number>("jwt.accessExpiresIn"),
        algorithm: "RS256",
      },
    );

    return accessToken;
  }
}
