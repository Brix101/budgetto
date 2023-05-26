import { authOptions } from "@/app/api/auth/[...nextauth]/route";
import { baseApi } from "@/constant/server";
import { categoriesSchema } from "@/schema/categories.schema";
import { getServerSession } from "next-auth";

async function getData() {
  const session = await getServerSession(authOptions);
  const res = await fetch(`${baseApi}/transactions`, {
    credentials: "include",
    headers: {
      Authorization: `Bearer ${session?.user.accessToken}`,
      cookie: `x-refresh=${session?.user.refreshToken}`,
    },
  });
  // The return value is *not* serialized
  // You can return Date, Map, Set,
  // Recommendation: handle errors
  if (!res.ok) {
    // This will activate the closest `error.js` Error Boundary
    throw new Error("Failed to fetch data");
  }

  return res.json().then((data) => data);
}

export default async function Page() {
  const { categories } = await getData();
  return (
    <div className="flex flex-col justify-between items-center p-24 min-h-screen">
      Transactions
    </div>
  );
}
