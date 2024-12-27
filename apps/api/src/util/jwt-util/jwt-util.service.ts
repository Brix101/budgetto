import { Injectable, UnauthorizedException } from "@nestjs/common";
import { ConfigService } from "@nestjs/config";
import { JwtService } from "@nestjs/jwt";
import { jwtConstants } from "src/auth/auth.constants";
import { CacheService } from "src/cache/cache.service";
import { JwtConfig } from "src/config/jwt.config";
import { User, UserDto } from "src/users/entities/user.entity";

import { TokenPairDto } from "../dto/token-pair.dto";

@Injectable()
export class JwtUtilService {
  constructor(
    private readonly jwtService: JwtService,
    private readonly cacheService: CacheService,
    private readonly configService: ConfigService,
  ) {}

  async genTokenPair(user: User): Promise<TokenPairDto> {
    const userUUID = user.generateUUID();
    const userObj = user.toObject();

    const accessToken = await this.signAccessToken(userUUID, userObj);
    const refreshToken = await this.signRefreshToken(userUUID);

    return {
      accessToken,
      refreshToken,
      expiresIn: jwtConstants.accessExpiresIn,
    };
  }

  async refreshAccessToken(refreshToken: string): Promise<TokenPairDto> {
    const jwtConfig = this.configService.get<JwtConfig>("jwt");

    const payload = await this.jwtService.verifyAsync<{ sub: string }>(
      refreshToken,
      {
        publicKey: jwtConfig.refreshPublicKey,
        algorithms: ["RS256"],
      },
    );

    const user = await this.cacheService.get<UserDto>(
      jwtConstants.keyPrefix + payload.sub,
    );

    if (!user) {
      throw new UnauthorizedException();
    }

    const accessToken = await this.signAccessToken(payload.sub, user);

    return {
      accessToken,
      refreshToken,
      expiresIn: jwtConstants.accessExpiresIn,
    };
  }

  async getUserObject(sub: string) {
    return this.cacheService.get<UserDto>(jwtConstants.keyPrefix + sub);
  }

  private async setUserObject(sub: string, payload: UserDto) {
    return this.cacheService.set(
      jwtConstants.keyPrefix + sub,
      payload,
      jwtConstants.accessExpiresIn * 1000,
    );
  }

  private async signAccessToken(sub: string, payload: UserDto) {
    const jwtConfig = this.configService.get<JwtConfig>("jwt");

    const userObj = await this.setUserObject(sub, payload);

    return this.jwtService.signAsync(
      {
        sub,
        email: userObj.email,
        name: userObj.name,
      },
      {
        privateKey: jwtConfig.accessPrivateKey,
        expiresIn: jwtConstants.accessExpiresIn,
        algorithm: "RS256",
      },
    );
  }

  private async signRefreshToken(sub: string) {
    const jwtConfig = this.configService.get<JwtConfig>("jwt");

    return this.jwtService.signAsync(
      { sub },
      {
        privateKey: jwtConfig.refreshPrivateKey,
        expiresIn: jwtConfig.refreshExpiresIn,
        algorithm: "RS256",
      },
    );
  }
}
