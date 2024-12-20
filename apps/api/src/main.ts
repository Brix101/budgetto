import { ConfigService } from "@nestjs/config";
import { NestFactory } from "@nestjs/core";

import type { ConfigSchema } from "./config/config.schema";
import { AppModule } from "./app.module";

async function bootstrap() {
  const app = await NestFactory.create(AppModule);
  const configService = app.get(ConfigService);
  const port = configService.get<ConfigSchema["PORT"]>("PORT");

  await app.listen(port);
}
bootstrap();
