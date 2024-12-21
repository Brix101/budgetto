import { EntityManager, MikroORM } from "@mikro-orm/core";
import {
  Injectable,
  InternalServerErrorException,
  Logger,
  NotFoundException,
} from "@nestjs/common";
import { UserDto } from "src/users/entities/user.entity";
import { UsersService } from "src/users/users.service";

import { CreateCategoryDto, UpdateCategoryDto } from "@budgetto/schema";

import { Category } from "./entities/category.entity";

@Injectable()
export class CategoriesService {
  private logger = new Logger(CategoriesService.name);

  constructor(
    private readonly orm: MikroORM,
    private readonly em: EntityManager,
    private readonly usersService: UsersService,
  ) {}

  async create(user: UserDto, createCategoryDto: CreateCategoryDto) {
    try {
      const userEntity = await this.usersService.findOne({ id: user.id });

      const category = this.em.create("Category", {
        ...createCategoryDto,
        user: userEntity,
      });

      await this.em.persistAndFlush(category);

      return category;
    } catch (error) {
      this.logger.error(error);

      throw new InternalServerErrorException(error);
    }
  }

  async findAll(user: UserDto) {
    try {
      const categories = await this.em.find(Category, {
        $or: [{ user: { id: user.id } }, { user: null }],
      });
      this.logger.log(categories);
      return categories;
    } catch (error) {
      this.logger.error(error);

      throw new InternalServerErrorException(error);
    }
  }

  findOne(id: number) {
    try {
      const category = this.em.findOneOrFail(Category, { id });

      return category;
    } catch (error) {
      this.logger.error(error);
      throw new NotFoundException([
        {
          validation: "id",
          message: "Category not found",
          path: ["id"],
        },
      ]);
    }
  }

  update(id: number, updateCategoryDto: UpdateCategoryDto) {
    return `This action updates a #${id} category`;
  }

  remove(id: number) {
    return `This action removes a #${id} category`;
  }
}
