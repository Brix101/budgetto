import { Injectable } from "@nestjs/common";
import { ConfigService } from "@nestjs/config";
import { PassportStrategy } from "@nestjs/passport";
import { ExtractJwt, Strategy } from "passport-jwt";
import { AccessPayloadDto } from "src/util/dto/payload.dto";
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

  async validate({ sub }: AccessPayloadDto) {
    const cachedUser = await this.jwtUtilService.getUserObject(sub);

    if (!cachedUser) {
      return null;
    }

    return { sub, ...cachedUser };
  }
}
