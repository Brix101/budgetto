import {
  FilterQuery,
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
import { UserRepository } from "./users.repository";

@Injectable()
export class UsersService {
  private logger = new Logger(UsersService.name);

  constructor(private readonly repo: UserRepository) {}

  async create(createUserDto: CreateUserDto) {
    try {
      const user = this.repo.create(createUserDto);
      await this.repo.insert(user);

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

  async findOne(where: FilterQuery<NoInfer<User>>) {
    try {
      const user = await this.repo.findOneOrFail(where);

      return user;
    } catch {
      const validation = Object.keys(where)[0];

      throw new NotFoundException([
        {
          validation: validation,
          message: "User not found",
          path: [validation],
        },
      ]);
    }
  }

  async update(id: number, updateUserDto: UpdateUserDto) {
    try {
      const user = await this.findOne({ id });
      user.assign(updateUserDto);

      await this.repo.nativeUpdate({ id }, updateUserDto);

      return user;
    } catch (error) {
      if (error instanceof NotFoundException) {
        throw error;
      }
      this.logger.error(error);
      throw new InternalServerErrorException(error);
    }
  }

  remove(id: number) {
    return `This action removes a #${id} user`;
  }
}
