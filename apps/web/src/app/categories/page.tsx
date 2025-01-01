import { auth } from "auth";

import { Card } from "~/components/ui/card";

export default async function CategoriesPage() {
  const session = await auth();

  console.log("CATEGORIES ++++++++++++++++++", session);

  // const res = await fetch(`${env.API_BASE_URL}/api/categories`, {
  //   method: "GET",
  //   headers: {
  //     "Content-Type": "application/json",
  //     "Access-Control-Allow-Origin": "*",
  //     Authorization: `Bearer ${session?.user.accessToken}`,
  //   },
  // });

  // const categories = (await res.json()) as CategoryDto[];

  return (
    <div className=" flex items-center justify-center min-h-screen">
      <Card className="p-10">
        <h1 className="text-2xl font-bold">Categories</h1>
        <ul>
          {/* {categories?.map((category) => (
            <li key={category.id}>{category.name} </li>
          ))} */}
        </ul>
      </Card>
    </div>
  );
}
