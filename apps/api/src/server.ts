import * as trpcExpress from "@trpc/server/adapters/express";
import cookieParser from "cookie-parser";
import cors from "cors";
import express from "express";
import morgan from "morgan";

import { createTRPCContext, createTrpcRouter, publicProcedure } from "./trpc";

const app = express();

export const appRouter = createTrpcRouter({
  greeting: publicProcedure.query(() => "hello tRPC v11!"),
});

export type AppRouter = typeof appRouter;

app.use(morgan("dev"));
app.use(
  "/api/trpc",
  cors({
    maxAge: 86400,
    credentials: true,
    origin: "http://localhost:3000",
  }),
  cookieParser(),
  trpcExpress.createExpressMiddleware({
    router: appRouter,
    createContext: createTRPCContext,
  }),
);

export const startServer = async () => {
  const port = process.env.PORT ?? 5000;

  const server = app.listen(port, () => {
    console.log(`server started on http://localhost:${port}/api`);
  });

  //////////////////////////////////////////////////////////////////////
  const signals = ["SIGTERM", "SIGINT", "SIGHUP", "SIGBREAK"];
  const errorTypes = ["unhandledRejection", "uncaughtException"];

  errorTypes.forEach((type) => {
    process.on(type, (error) => {
      try {
        console.error(`process exit due to ${type}`);
        console.error(error);
        process.exit(1);
      } catch {
        process.exit(1);
      }
    });
  });

  signals.forEach((type) => {
    process.on(type, () => {
      console.log(`${type} signal received.`);
      console.log("Closing http server.");
      server.close((err) => {
        console.log("Http server closed.");
        process.exit(err ? 1 : 0);
      });
    });
  });
};
