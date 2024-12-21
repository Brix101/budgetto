import {
  Body,
  Controller,
  Delete,
  Get,
  Param,
  Patch,
  Post,
  Request,
  UseGuards,
  UsePipes,
} from "@nestjs/common";
import { AppAbility } from "src/casl/casl-ability.factory/casl-ability.factory";
import Action from "src/casl/casl-action.enum";
import { CheckPolicies } from "src/casl/policies/policies.decorator";
import { PoliciesGuard } from "src/casl/policies/policies.guard";
import { ZodValidationPipe } from "src/common/zod-validation.pipe";
import { UserDto } from "src/users/entities/user.entity";

import type { CreateCategoryDto, UpdateCategoryDto } from "@budgetto/schema";
import { createCategorySchema, updateCategorySchema } from "@budgetto/schema";

import { CategoriesService } from "./categories.service";
import { Category } from "./entities/category.entity";

@Controller("categories")
export class CategoriesController {
  constructor(private readonly categoriesService: CategoriesService) {}

  @Post()
  @UsePipes(new ZodValidationPipe(createCategorySchema))
  create(
    @Body() createCategoryDto: CreateCategoryDto,
    @Request() req: { user: UserDto },
  ) {
    return this.categoriesService.create(req.user, createCategoryDto);
  }

  @Get()
  @UseGuards(PoliciesGuard)
  @CheckPolicies((ability: AppAbility) => ability.can(Action.Read, Category))
  findAll(@Request() req: { user: UserDto }) {
    return this.categoriesService.findAll(req.user);
  }

  @Get(":id")
  @UseGuards(PoliciesGuard)
  @CheckPolicies((ability: AppAbility) => ability.can(Action.Read, Category))
  findOne(@Param("id") id: string) {
    return this.categoriesService.findOne(+id);
  }

  @Patch(":id")
  @UsePipes(new ZodValidationPipe(updateCategorySchema))
  update(
    @Param("id") id: string,
    @Body() updateCategoryDto: UpdateCategoryDto,
  ) {
    return this.categoriesService.update(+id, updateCategoryDto);
  }

  @Delete(":id")
  remove(@Param("id") id: string) {
    return this.categoriesService.remove(+id);
  }
}
