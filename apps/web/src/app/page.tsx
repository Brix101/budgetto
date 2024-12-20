import { SignInForm } from "~/sign-in-form";

export const runtime = "edge";

export default function Home() {
  return (
    <div className="items-center justify-items-center min-h-screen flex w-full">
      <SignInForm />
    </div>
  );
}
