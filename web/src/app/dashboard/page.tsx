import { getServerSession } from "next-auth/next";
import { authOptions } from "@/app/api/auth/[...nextauth]/route";

export default async function Page() {
  const session = await getServerSession(authOptions);

  console.log({ session });
  return (
    <section className="flex flex-col justify-between items-center p-24 min-h-screen">
      Dashboard
    </section>
  );
}
