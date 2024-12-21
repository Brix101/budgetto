import { Body, Controller, Patch, Request, UsePipes } from "@nestjs/common";
import { ZodValidationPipe } from "src/common/zod-validation.pipe";

import type { UpdateUserDto } from "@budgetto/schema";
import { updateUserSchema } from "@budgetto/schema";

import { UserDto } from "./entities/user.entity";
import { UsersService } from "./users.service";

@Controller("users")
export class UsersController {
  constructor(private readonly usersService: UsersService) {}

  // @Public()
  // @Post()
  // @UsePipes(new ZodValidationPipe(createUserSchema))
  // create(@Body() createUserDto: CreateUserDto) {
  //   return this.usersService.create(createUserDto);
  // }

  // @Get()
  // findAll() {
  //   return this.usersService.findAll();
  // }

  // @Get(":id")
  // findOne(@Param("id") id: string) {
  //   return this.usersService.findOne(+id);
  // }

  @Patch()
  @UsePipes(new ZodValidationPipe(updateUserSchema))
  update(
    // @Param("id") id: string,
    @Body() updateUserDto: UpdateUserDto,
    @Request() req: { user: UserDto },
  ) {
    return this.usersService.update(req.user.id, updateUserDto);
  }

  // @Delete(":id")
  // remove(@Param("id") id: string) {
  //   return this.usersService.remove(+id);
  // }
}
