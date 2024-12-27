import type { UserDto } from "src/users/entities/user.entity";

export class UserAuthDto {
  user: UserDto;
  accessToken: string;
  refreshToken: string;
  expiresIn: number;
}
