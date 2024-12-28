import { SignInForm } from "~/components/sign-in-form";
import { Card } from "~/components/ui/card";

export default function Home() {
  return (
    <div className=" flex items-center justify-center min-h-screen">
      <Card className="p-10">
        <SignInForm />
      </Card>
    </div>
  );
}
