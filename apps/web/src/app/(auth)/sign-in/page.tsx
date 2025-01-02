import { SignInForm } from "~/components/sign-in-form";
import { Card } from "~/components/ui/card";

export default function Home() {
  return (
    <div className="flex min-h-screen items-center justify-center">
      <Card className="p-10">
        <SignInForm />
      </Card>
    </div>
  );
}
