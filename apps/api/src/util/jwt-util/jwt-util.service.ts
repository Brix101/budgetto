import { Injectable, UnauthorizedException } from "@nestjs/common";
import { ConfigService } from "@nestjs/config";
import { JwtService } from "@nestjs/jwt";
import { CacheService } from "src/cache/cache.service";
import { JwtConfig } from "src/config/jwt.config";
import { User, UserDto } from "src/users/entities/user.entity";

import { UserAuthDto } from "../dto/auth.dto";
import { AccessPayloadDto, RefreshPayloadDto } from "../dto/payload.dto";

@Injectable()
export class JwtUtilService {
  private readonly EXPIRES_IN = 3600;
  private readonly CACHE_KEY = "AUTH:";

  constructor(
    private readonly jwtService: JwtService,
    private readonly cacheService: CacheService,
    private readonly configService: ConfigService,
  ) {}

  async getTokenPair(user: User): Promise<UserAuthDto> {
    const userUUID = user.generateUUID();
    const payload = user.toObject();

    const accessToken = await this.signAccessToken(userUUID, payload);
    const refreshToken = await this.signRefreshToken(userUUID, payload);

    return {
      user: payload,
      accessToken,
      refreshToken,
      expiresIn: this.EXPIRES_IN,
    };
  }

  async refreshAccessToken(refreshToken: string): Promise<UserAuthDto> {
    try {
      const jwtConfig = this.configService.get<JwtConfig>("jwt");

      const { sub } = await this.jwtService.verifyAsync<RefreshPayloadDto>(
        refreshToken,
        {
          publicKey: jwtConfig.refreshPublicKey,
          algorithms: ["RS256"],
        },
      );

      const userObj = await this.getUserObject(sub);

      if (!userObj) {
        throw new UnauthorizedException();
      }

      const accessToken = await this.signAccessToken(sub, userObj);

      return {
        user: userObj,
        accessToken,
        refreshToken,
        expiresIn: this.EXPIRES_IN,
      };
    } catch {
      throw new UnauthorizedException([
        {
          code: "invalid_token",
          message: "Invalid refresh token",
          path: ["refreshToken"],
        },
      ]);
    }
  }

  async getUserObject(sub: string) {
    return this.cacheService.get<UserDto>(this.CACHE_KEY + sub);
  }

  private async signAccessToken(sub: string, userObj: UserDto) {
    const jwtConfig = this.configService.get<JwtConfig>("jwt");
    const payload: AccessPayloadDto = {
      sub,
      email: userObj.email,
      name: userObj.name,
    };

    return this.jwtService.signAsync(payload, {
      privateKey: jwtConfig.accessPrivateKey,
      expiresIn: jwtConfig.accessExpiresIn,
      algorithm: "RS256",
    });
  }

  private async signRefreshToken(sub: string, userObj: UserDto) {
    const jwtConfig = this.configService.get<JwtConfig>("jwt");

    await this.cacheService.set(
      this.CACHE_KEY + sub,
      userObj,
      jwtConfig.refreshExpiresIn,
    );

    const payload: RefreshPayloadDto = { sub };

    return this.jwtService.signAsync(payload, {
      privateKey: jwtConfig.refreshPrivateKey,
      expiresIn: jwtConfig.refreshExpiresIn,
      algorithm: "RS256",
    });
  }
}
