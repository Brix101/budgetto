import {
  Body,
  Controller,
  Delete,
  Get,
  Param,
  Patch,
  Post,
  UsePipes,
} from "@nestjs/common";
import { ZodValidationPipe } from "src/common/zod-validation.pipe";

import type { CreateCategoryDto, UpdateCategoryDto } from "@budgetto/schema";
import { createCategorySchema, updateCategorySchema } from "@budgetto/schema";

import { CategoriesService } from "./categories.service";

@Controller("categories")
export class CategoriesController {
  constructor(private readonly categoriesService: CategoriesService) {}

  @Post()
  @UsePipes(new ZodValidationPipe(createCategorySchema))
  create(@Body() createCategoryDto: CreateCategoryDto) {
    return this.categoriesService.create(createCategoryDto);
  }

  @Get()
  findAll() {
    return this.categoriesService.findAll();
  }

  @Get(":id")
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
