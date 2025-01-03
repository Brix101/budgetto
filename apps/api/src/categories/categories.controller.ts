import {
  Body,
  Controller,
  Delete,
  ForbiddenException,
  Get,
  Param,
  Patch,
  Post,
  Request,
  UsePipes,
} from "@nestjs/common";
import { CaslAbilityFactory } from "src/casl/casl-ability.factory/casl-ability.factory";
import Action from "src/casl/casl-action.enum";
import { ZodValidationPipe } from "src/common/zod-validation.pipe";
import { User, UserDto } from "src/users/entities/user.entity";

import type {
  CreateCategoryDto,
  UpdateCategoryDto,
} from "@budgetto/schema/category";
import {
  createCategorySchema,
  updateCategorySchema,
} from "@budgetto/schema/category";

import { CategoriesService } from "./categories.service";

@Controller("categories")
export class CategoriesController {
  constructor(
    private readonly categoriesService: CategoriesService,
    private readonly caslAbilityFactory: CaslAbilityFactory,
  ) {}

  @Post()
  @UsePipes(new ZodValidationPipe(createCategorySchema))
  create(
    @Body() createCategoryDto: CreateCategoryDto,
    @Request() req: { user: UserDto },
  ) {
    return this.categoriesService.create(req.user, createCategoryDto);
  }

  @Get()
  findAll(@Request() req: { user: UserDto }) {
    return this.categoriesService.findAll(req.user);
  }

  @Get(":id")
  async findOne(@Param("id") id: string, @Request() req: { user: User }) {
    const category = await this.categoriesService.findOne(+id);

    const ability = this.caslAbilityFactory.createForUser(req.user);

    category.assign({ user: { id: category.user.id } });

    console.log(category);
    const hasAbility = ability.can(Action.Manage, category);

    if (!hasAbility) {
      throw new ForbiddenException(
        "You do not have permission to view this category",
      );
    }

    return category;
  }

  @Patch(":id")
  @UsePipes(new ZodValidationPipe(updateCategorySchema))
  update(
    @Param("id") id: string,
    @Body() updateCategoryDto: UpdateCategoryDto,
    @Request() _req: { user: UserDto },
  ) {
    return this.categoriesService.update(+id, updateCategoryDto);
  }

  @Delete(":id")
  remove(@Param("id") id: string, @Request() _req: { user: UserDto }) {
    return this.categoriesService.remove(+id);
  }
}
