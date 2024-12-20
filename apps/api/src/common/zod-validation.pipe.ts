import type { ArgumentMetadata } from "@nestjs/common";
import { BadRequestException, Injectable } from "@nestjs/common";
import { ZodSchema } from "zod";

@Injectable()
export class ZodValidationPipe {
  constructor(private schema: ZodSchema) {}

  transform(value: unknown, _metadata: ArgumentMetadata) {
    try {
      return this.schema.parse(value) as unknown;
    } catch (error) {
      throw new BadRequestException(error);
    }
  }
}
