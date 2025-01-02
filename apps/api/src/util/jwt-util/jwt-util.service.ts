import {
  Injectable,
  InternalServerErrorException,
  Logger,
  UnauthorizedException,
} from "@nestjs/common";
import { ConfigService } from "@nestjs/config";
import { JsonWebTokenError, JwtService } from "@nestjs/jwt";
import { CacheService } from "src/cache/cache.service";
import { JwtConfig } from "src/config/jwt.config";
import { User } from "src/users/entities/user.entity";

import { SignInResponseDto, UserSchemaDto } from "@budgetto/schema";

import { AccessPayloadDto, RefreshPayloadDto } from "../dto/payload.dto";

@Injectable()
export class JwtUtilService {
  private readonly logger = new Logger(JwtUtilService.name);
  private readonly CACHE_KEY = "AUTH:";

  constructor(
    private readonly jwtService: JwtService,
    private readonly cacheService: CacheService,
    private readonly configService: ConfigService,
  ) {}

  async getTokenPair(user: User): Promise<SignInResponseDto> {
    const userUUID = user.generateUUID();
    const userObj = user.toObject();

    const payload: UserSchemaDto = {
      ...userObj,
      createdAt: user.createdAt.toISOString(),
      updatedAt: user.updatedAt.toISOString(),
    };

    const accessToken = await this.signAccessToken(userUUID, payload);
    const refreshToken = await this.signRefreshToken(userUUID, payload);

    return {
      user: payload,
      accessToken,
      refreshToken,
    };
  }

  async refreshToken(refreshToken: string): Promise<SignInResponseDto> {
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
      };
    } catch (error) {
      if (
        error instanceof UnauthorizedException ||
        error instanceof JsonWebTokenError
      ) {
        throw new UnauthorizedException([
          {
            code: "invalid_token",
            message: error.message || "Invalid or expired token",
            path: ["refreshToken"],
          },
        ]);
      }

      this.logger.error(error);
      throw new InternalServerErrorException([
        {
          code: "internal_server_error",
          message:
            error instanceof Error ? error.message : "Something went wrong",
        },
      ]);
    }
  }

  async getUserObject(sub: string) {
    return this.cacheService.get<UserSchemaDto>(this.CACHE_KEY + sub);
  }

  private async signAccessToken(sub: string, userObj: UserSchemaDto) {
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

  private async signRefreshToken(sub: string, userObj: UserSchemaDto) {
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
