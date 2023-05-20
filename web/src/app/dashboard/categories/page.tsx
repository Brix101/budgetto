import { baseApi } from "@/constant/server";
import { categoriesSchema } from "@/schema/categories.schema";
import { getServerSession } from "next-auth/next";
import { authOptions } from "@/app/api/auth/[...nextauth]/route";

async function getData() {
  const session = await getServerSession(authOptions);
  const res = await fetch(`${baseApi}/categories`, {
    credentials: "include",
    headers: {
      Authorization: `Bearer ${session?.user.accessToken}`,
      cookie: `x-refresh=${session?.user.refreshToken}`,
    },
  });
  // The return value is *not* serialized
  // You can return Date, Map, Set, etc.

  // const xAccessToken = res.headers.get("x-access-token");
  // console.log({ xAccessToken });
  // Recommendation: handle errors
  if (!res.ok) {
    // This will activate the closest `error.js` Error Boundary
    throw new Error("Failed to fetch data");
  }

  return res.json().then((data) => categoriesSchema.parse(data));
}

export default async function Page() {
  const { categories } = await getData();

  return (
    <div className="flex flex-col justify-between p-24 min-h-screen">
      {categories?.map((category, index) => {
        return (
          <div key={index}>
            <h1 className="font-bold">{category.name}</h1>
            <span>{category.note}</span>
            <p>{category.isDefault ? "default" : "user created"}</p>
          </div>
        );
      })}
    </div>
  );
}
