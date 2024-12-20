import { Injectable } from "@nestjs/common";
import { JwtService } from "@nestjs/jwt";
import { User } from "src/users/entities/user.entity";
import { UsersService } from "src/users/users.service";
import { PasswordUtilService } from "src/util/password-util.service";

@Injectable()
export class AuthService {
  constructor(
    private readonly usersService: UsersService,
    private readonly passwordUtilService: PasswordUtilService,
    private jwtService: JwtService,
  ) {}

  async validateUser(email: string, pass: string) {
    try {
      const user = await this.usersService.findOneByEmail(email);

      const hashedPassword = await user.password.load();

      const isVerified = await this.passwordUtilService.verify(
        hashedPassword,
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

  async signIn(user: Partial<User>): Promise<any> {
    const payload = { sub: user.id, email: user.email };
    return {
      access_token: await this.jwtService.signAsync(payload),
    };
  }
}
