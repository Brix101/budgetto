import { auth } from "auth";

import type { CategoryDto } from "@budgetto/schema";
import { Card } from "@budgetto/ui/card";

import { env } from "~/env";

async function getCategories(): Promise<CategoryDto[] | null> {
  try {
    const session = await auth();

    const response = await fetch(`${env.API_BASE_URL}/api/categories`, {
      method: "GET",
      headers: {
        "Content-Type": "application/json",
        "Access-Control-Allow-Origin": "*",
        Authorization: `Bearer ${session?.user.accessToken}`,
      },
    });

    const result = (await response.json()) as CategoryDto[];

    return result;
  } catch (e) {
    console.log(e);
    return null;
  }
}

export default async function CategoriesPage() {
  const categories = await getCategories();

  return (
    <div className="flex min-h-screen items-center justify-center">
      <Card className="p-10">
        <h1 className="text-2xl font-bold">Categories</h1>
        <ul>
          {categories?.map((category) => (
            <li key={category.id}>{category.name} </li>
          ))}
        </ul>
      </Card>
    </div>
  );
}
