import { UniqueConstraintViolationException } from "@mikro-orm/core";
import {
  BadRequestException,
  Injectable,
  InternalServerErrorException,
  Logger,
  NotFoundException,
} from "@nestjs/common";
import { UserDto } from "src/users/entities/user.entity";

import { CreateCategoryDto, UpdateCategoryDto } from "@budgetto/schema";

import { CategoryRepository } from "./categories.repository";

@Injectable()
export class CategoriesService {
  private logger = new Logger(CategoriesService.name);

  constructor(private readonly repo: CategoryRepository) {}

  async create(user: UserDto, createCategoryDto: CreateCategoryDto) {
    try {
      const category = this.repo.create({
        ...createCategoryDto,
        user: user.id,
      });

      await this.repo.insert(category);

      return category;
    } catch (error) {
      if (error instanceof UniqueConstraintViolationException) {
        throw new BadRequestException([
          {
            validation: "name",
            code: "invalid_name",
            message: "Category with this name already exists",
            path: ["name"],
          },
        ]);
      }

      this.logger.error(error);

      throw new InternalServerErrorException(error);
    }
  }

  async findAll(user: UserDto) {
    try {
      const categories = await this.repo.findAll({
        where: {
          $or: [{ user: { id: user.id } }, { user: null }],
        },
      });
      return categories;
    } catch (error) {
      this.logger.error(error);

      throw new InternalServerErrorException(error);
    }
  }

  async findOne(id: number) {
    try {
      const category = await this.repo.findOneOrFail({ id });

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
      const category = await this.findOne(id);
      category.assign(updateCategoryDto);

      await this.repo.nativeUpdate({ id }, updateCategoryDto);

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
      const category = await this.findOne(id);

      await this.repo.nativeDelete({ id });

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
