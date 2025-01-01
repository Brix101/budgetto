import { env } from "process";
import { headers } from "next/headers";

import { SignInForm } from "~/components/sign-in-form";
import { Card } from "~/components/ui/card";

export default async function Home() {
  const res = await fetch(`${env.NEXT_PUBLIC_BASE_URL}/api/auth/refresh`, {
    method: "POST",
    headers: await headers(),
    body: JSON.stringify({ userId: "1" }),
  });

  const data = await res.json();

  console.log("+++++++++++++++++++++++++++++++++++++++++++");
  console.log(data);

  return (
    <div className=" flex items-center justify-center min-h-screen">
      <Card className="p-10">
        <SignInForm />
      </Card>
    </div>
  );
}
