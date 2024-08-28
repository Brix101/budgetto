import { initTRPC } from "@trpc/server";
import * as trpcExpress from "@trpc/server/adapters/express";
import superjson from "superjson";
import { ZodError } from "zod";

export const createContext = ({
  req,
  res,
}: trpcExpress.CreateExpressContextOptions) => {
  return { req, res };
};

type TrpcContext = Awaited<ReturnType<typeof createContext>> & {};

export const t = initTRPC.context<TrpcContext>().create({
  transformer: superjson,
  errorFormatter: ({ shape, error }) => ({
    ...shape,
    data: {
      ...shape.data,
      zodError: error.cause instanceof ZodError ? error.cause.flatten() : null,
    },
  }),
});

export const createTrpcRouter = t.router;
export const publicProcedure = t.procedure;
export const privateProcedure = t.procedure.use(async (opts) => {
  const { ctx } = opts;
  console.log(ctx);

  return opts.next(opts);
});
