import { NextApiRequest } from "next";

export const runtime = "edge";

/**
 * Configure basic CORS headers
 * You should extend this to match your needs
 */
const setCorsHeaders = (res: Response) => {
  res.headers.set("Access-Control-Allow-Origin", "*");
  res.headers.set("Access-Control-Request-Method", "*");
  res.headers.set("Access-Control-Allow-Methods", "OPTIONS, GET, POST");
  res.headers.set("Access-Control-Allow-Headers", "*");
};

export const OPTIONS = () => {
  const response = new Response(null, {
    status: 204,
  });
  setCorsHeaders(response);
  return response;
};

const handler = async (req: NextApiRequest) => {
  const path = req.url?.split("/api/trpc/")[1];
  const response = await fetch(`http://localhost:5000/api/trpc/${path}`, {
    credentials: "include",
    method: req.method,
    headers: req.headers as HeadersInit,
    body: req.body,
  });

  setCorsHeaders(response);
  return response;
};

export { handler as GET, handler as POST };
