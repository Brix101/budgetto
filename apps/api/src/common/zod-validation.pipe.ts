import type { ArgumentMetadata, PipeTransform } from "@nestjs/common";
import type { ZodSchema } from "zod";
import { BadRequestException, Logger } from "@nestjs/common";

export class ZodValidationPipe implements PipeTransform {
  private logger = new Logger(ZodValidationPipe.name);
  constructor(private schema: ZodSchema) {}

  transform(value: unknown, _metadata: ArgumentMetadata) {
    try {
      const parsedValue = this.schema.parse(value);
      return parsedValue;
    } catch (error) {
      this.logger.error(error);
      throw new BadRequestException(error);
    }
  }
}
