import { redirect } from "next/navigation";
import { auth } from "auth";

import { SignInForm } from "~/components/sign-in-form";
import { Card } from "~/components/ui/card";

export default async function Home() {
  const session = await auth();

  if (session) {
    redirect("/dashboard");
  } else {
    redirect("/sign-in");
  }

  return (
    <div className="flex min-h-screen items-center justify-center">
      <Card className="p-10">
        <SignInForm />
      </Card>
    </div>
  );
}
