import { Injectable } from "@nestjs/common";
import { PassportStrategy } from "@nestjs/passport";
import { ExtractJwt, Strategy } from "passport-jwt";
import { jwtConstants } from "src/auth/auth.constants";
import { CacheService } from "src/cache/cache.service";
import { UserPayloadDto } from "src/users/dto/user-payload.dto";
import { UserDto } from "src/users/entities/user.entity";

@Injectable()
export class JwtStrategy extends PassportStrategy(Strategy) {
  constructor(private readonly cacheService: CacheService) {
    super({
      jwtFromRequest: ExtractJwt.fromAuthHeaderAsBearerToken(),
      ignoreExpiration: false,
      secretOrKey: jwtConstants.secret,
    });
  }

  async validate(payload: UserPayloadDto) {
    const cachedUser = await this.cacheService.get<UserDto>(
      jwtConstants.keyPrefix + payload.sub,
    );

    if (!cachedUser) {
      return null;
    }

    return { sub: payload.sub, ...cachedUser };
  }
}
