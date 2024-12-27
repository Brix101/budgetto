import { Injectable } from "@nestjs/common";
import { ConfigService } from "@nestjs/config";
import * as argon2 from "argon2";

@Injectable()
export class PasswordUtilService {
  constructor(private readonly configService: ConfigService) {}

  async hash(password: string): Promise<string> {
    const secret = this.configService.get<string>("password.secret");

    return argon2.hash(password, {
      secret: Buffer.from(secret),
    });
  }

  async verify(hashedPassword: string, password: string): Promise<boolean> {
    const secret = this.configService.get<string>("password.secret");

    return argon2.verify(hashedPassword, password, {
      secret: Buffer.from(secret),
    });
  }
}
