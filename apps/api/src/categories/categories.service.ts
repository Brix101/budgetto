import { EntityManager, MikroORM } from "@mikro-orm/core";
import {
  Injectable,
  InternalServerErrorException,
  Logger,
  NotFoundException,
} from "@nestjs/common";
import { UserDto } from "src/users/entities/user.entity";

import { CreateCategoryDto, UpdateCategoryDto } from "@budgetto/schema";

import { Category } from "./entities/category.entity";

@Injectable()
export class CategoriesService {
  private logger = new Logger(CategoriesService.name);

  constructor(
    private readonly orm: MikroORM,
    private readonly em: EntityManager,
  ) {}

  async create(user: UserDto, createCategoryDto: CreateCategoryDto) {
    try {
      const category = this.em.create(Category, {
        ...createCategoryDto,
        user: user.id,
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

  async update(id: number, updateCategoryDto: UpdateCategoryDto) {
    try {
      const category = this.findOne(id);
      this.em.assign(category, updateCategoryDto);

      await this.em.nativeUpdate(Category, { id }, updateCategoryDto);

      return category;
    } catch (error) {
      if (error instanceof NotFoundException) {
        throw error;
      }
      this.logger.error(error);
      throw new InternalServerErrorException(error);
    }
  }

  async remove(id: number) {
    try {
      const category = this.findOne(id);

      await this.em.nativeDelete(Category, { id });

      return category;
    } catch (error) {
      if (error instanceof NotFoundException) {
        throw error;
      }
      this.logger.error(error);
      throw new InternalServerErrorException(error);
    }
  }
}
