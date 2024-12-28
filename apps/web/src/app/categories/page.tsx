import { auth } from "auth";

import { Card } from "~/components/ui/card";

export default async function CategoriesPage() {
  const session = await auth();
  console.log("session (server side)", session);
  return (
    <div className=" flex items-center justify-center min-h-screen">
      <Card className="p-10"></Card>
    </div>
  );
}
