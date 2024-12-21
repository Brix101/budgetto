import {
  EntityManager,
  FilterQuery,
  MikroORM,
  UniqueConstraintViolationException,
} from "@mikro-orm/core";
import {
  BadRequestException,
  Injectable,
  InternalServerErrorException,
  Logger,
  NotFoundException,
} from "@nestjs/common";

import { CreateUserDto, UpdateUserDto } from "@budgetto/schema";

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
            code: "invalid_email",
            message: "User with this email already exists",
            path: ["email"],
          },
        ]);
      }

      this.logger.error(error);
      throw new InternalServerErrorException(error || "Something went wrong");
    }
  }

  findAll() {
    return `This action returns all user`;
  }

  findOne(where: FilterQuery<NoInfer<User>>) {
    try {
      return this.em.findOneOrFail(User, where);
    } catch {
      throw new NotFoundException([
        {
          validation: "id",
          message: "User not found",
          path: ["id"],
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
