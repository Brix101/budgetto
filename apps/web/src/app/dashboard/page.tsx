import { auth, signOut } from "auth";

import type { CategoryDto } from "@budgetto/schema/category";

import { Button } from "~/components/ui/button";
import { Card } from "~/components/ui/card";
import { env } from "~/env";

export const dynamic = "force-dynamic";

async function getCategories(): Promise<CategoryDto[]> {
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
    return [];
  }
}

export default async function CategoriesPage() {
  const categories = await getCategories();

  return (
    <div className="flex min-h-screen flex-col items-center justify-center">
      <Button
        onClick={async () => {
          "use server";

          console.log("signing out");
          await signOut();
        }}
      >
        Sign Out
      </Button>
      <Card className="p-10">
        <h1 className="text-2xl font-bold">Categories</h1>
        <ul>
          {categories.map((category) => (
            <li key={category.id}>{category.name} </li>
          ))}
        </ul>
      </Card>
    </div>
  );
}
