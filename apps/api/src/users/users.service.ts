import {
  EntityManager,
  MikroORM,
  UniqueConstraintViolationException,
} from "@mikro-orm/core";
import {
  BadRequestException,
  Injectable,
  InternalServerErrorException,
  Logger,
} from "@nestjs/common";

import { CreateUserDto } from "./dto/create-user.dto";
import { UpdateUserDto } from "./dto/update-user.dto";
import { User } from "./entities/user.entity";

@Injectable()
export class UsersService {
  private logger = new Logger(UsersService.name);

  constructor(
    private readonly orm: MikroORM,
    private readonly em: EntityManager,
  ) {}

  async create(createUserDto: CreateUserDto) {
    try {
      const user = this.em.create(User, createUserDto);
      await this.em.persistAndFlush(user);

      return user;
    } catch (error) {
      if (error instanceof UniqueConstraintViolationException) {
        throw new BadRequestException([
          {
            validation: "email",
            code: "invalid_string",
            message: "User with this email already exists",
            path: ["email"],
          },
        ]);
      }

      throw new InternalServerErrorException(error || "Something went wrong");
    }
  }

  findAll() {
    return `This action returns all user`;
  }

  findOne(id: number) {
    return `This action returns a #${id} user`;
  }

  async findOneByEmail(email: string) {
    try {
      return this.em.findOneOrFail(User, { email });
    } catch (error) {
      this.logger.error(error);
      throw new BadRequestException([
        {
          validation: "email",
          code: "invalid_string",
          message: "Invalid email or password",
          path: ["email"],
        },
      ]);
    }
  }

  update(id: number, updateUserDto: UpdateUserDto) {
    return `This action updates a #${id} user`;
  }

  remove(id: number) {
    return `This action removes a #${id} user`;
  }
}
