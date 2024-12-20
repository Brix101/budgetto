import { ConfigService } from "@nestjs/config";
import { NestFactory } from "@nestjs/core";

import type { ConfigSchema } from "./config/config.schema";
import { AppModule } from "./app.module";
import { HttpExceptionFilter } from "./http-exception.filter";

async function bootstrap() {
  const app = await NestFactory.create(AppModule);
  const configService = app.get(ConfigService);
  const port = configService.get<ConfigSchema["PORT"]>("PORT");

  // await app.get(MikroORM).getSchemaGenerator().ensureDatabase();
  // await app.get(MikroORM).getSchemaGenerator().updateSchema();

  app.setGlobalPrefix("api");
  app.useGlobalFilters(new HttpExceptionFilter());
  // Starts listening for shutdown hooks
  app.enableShutdownHooks();

  await app.listen(port);
}
bootstrap();
