import { Logger } from "@nestjs/common";
import { ConfigService } from "@nestjs/config";
import { NestFactory } from "@nestjs/core";
import morgan from "morgan";

import type { ServerConfig } from "./config/config.schema";
import { AppModule } from "./app.module";
import { HttpExceptionFilter } from "./http-exception.filter";

async function bootstrap() {
  const logger = new Logger("HTTP_REQUEST");

  const app = await NestFactory.create(AppModule);
  const configService = app.get(ConfigService);
  const port = configService.get<ServerConfig["PORT"]>("PORT");

  app.setGlobalPrefix("api");
  app.useGlobalFilters(new HttpExceptionFilter());
  app.use(
    morgan("tiny", {
      stream: {
        write: (message) => logger.log(message.replace("\n", "")),
      },
    }),
  );
  // Starts listening for shutdown hooks for graceful shutdown
  app.enableShutdownHooks();

  await app.listen(port);
}
bootstrap();
