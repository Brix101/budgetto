import { Injectable } from "@nestjs/common";
import { ConfigService } from "@nestjs/config";
import { PassportStrategy } from "@nestjs/passport";
import { ExtractJwt, Strategy } from "passport-jwt";
import { UserPayloadDto } from "src/users/dto/user-payload.dto";
import { JwtUtilService } from "src/util/jwt-util/jwt-util.service";

@Injectable()
export class JwtStrategy extends PassportStrategy(Strategy) {
  constructor(
    private readonly jwtUtilService: JwtUtilService,
    private readonly configService: ConfigService,
  ) {
    super({
      jwtFromRequest: ExtractJwt.fromAuthHeaderAsBearerToken(),
      ignoreExpiration: false,
      secretOrKey: configService.get<string>("jwt.accessPublicKey"),
    });
  }

  async validate(payload: UserPayloadDto) {
    const cachedUser = await this.jwtUtilService.getUserObject(payload.sub);

    if (!cachedUser) {
      return null;
    }

    return { sub: payload.sub, ...cachedUser };
  }
}
