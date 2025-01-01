import { cookies } from "next/headers";

import { env } from "~/env";

export async function POST(request: Request) {
  // const body = await request.json();

  console.log("Refresh token body");
  console.log("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa");
  const prefix = env.NODE_ENV === "development" ? "__Dev-" : "";

  const cookieStore = await cookies();

  const refreshToken = cookieStore.get(
    `${prefix}budgetto.refresh-token`,
  )?.value;

  // change it with your own endpoint
  const res = await fetch(`${env.API_BASE_URL}/api/auth/refresh`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
      "Access-Control-Allow-Origin": "*",
    },
    body: JSON.stringify({
      refreshToken: refreshToken,
    }),
  });

  const data = await res.json();

  console.log("Refreshed token");
  console.log(data);

  return Response.json({
    success: res.ok,
    status: res.status,
    data,
  });
}
