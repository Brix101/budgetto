import { Body, Controller, Param, Patch, Post, UsePipes } from "@nestjs/common";
import { ZodValidationPipe } from "src/common/zod-validation.pipe";

import type { CreateUserDto, UpdateUserDto } from "@budgetto/schema";
import { createUserSchema, updateUserSchema } from "@budgetto/schema";

import { UsersService } from "./users.service";

@Controller("users")
export class UsersController {
  constructor(private readonly usersService: UsersService) {}

  @Post()
  @UsePipes(new ZodValidationPipe(createUserSchema))
  create(@Body() createUserDto: CreateUserDto) {
    return this.usersService.create(createUserDto);
  }

  // @Get()
  // findAll() {
  //   return this.usersService.findAll();
  // }

  // @Get(":id")
  // findOne(@Param("id") id: string) {
  //   return this.usersService.findOne(+id);
  // }

  @Patch(":id")
  @UsePipes(new ZodValidationPipe(updateUserSchema))
  update(@Param("id") id: string, @Body() updateUserDto: UpdateUserDto) {
    return this.usersService.update(+id, updateUserDto);
  }

  // @Delete(":id")
  // remove(@Param("id") id: string) {
  //   return this.usersService.remove(+id);
  // }
}
