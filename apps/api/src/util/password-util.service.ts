import { Injectable } from "@nestjs/common";
import * as argon2 from "argon2";

@Injectable()
export class PasswordUtilService {
  private readonly secret = "randomsalt1234567890";

  constructor() {}

  async hash(password: string): Promise<string> {
    return argon2.hash(password, {
      secret: Buffer.from(this.secret),
    });
  }

  async verify(hashedPassword: string, password: string): Promise<boolean> {
    return argon2.verify(hashedPassword, password, {
      secret: Buffer.from(this.secret),
    });
  }
}
