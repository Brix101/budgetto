import { Injectable, UnauthorizedException } from "@nestjs/common";
import { UsersService } from "src/users/users.service";
import { PasswordUtilService } from "src/util/password-util.service";

@Injectable()
export class AuthService {
  constructor(
    private readonly usersService: UsersService,
    private readonly passwordUtilService: PasswordUtilService,
  ) {}

  async signIn(username: string, pass: string): Promise<any> {
    try {
      const user = await this.usersService.findOneByEmail(username);
      const hashedPassword = await user.password.load();

      const isVerified = await this.passwordUtilService.verify(
        hashedPassword,
        pass,
      );

      if (!isVerified) {
        throw new UnauthorizedException([
          {
            validation: "email",
            code: "invalid_string",
            message: "Invalid email or password",
            path: ["email"],
          },
        ]);
      }
      // TODO: Generate a JWT and return it here
      // instead of the user object
      return user;
    } catch {
      throw new UnauthorizedException([
        {
          validation: "email",
          code: "invalid_string",
          message: "Invalid email or password",
          path: ["email"],
        },
      ]);
    }
  }
}
