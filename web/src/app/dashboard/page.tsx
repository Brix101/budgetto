import { getServerAuthSession } from "@/utils/auth";

export default async function Page() {
  const session = await getServerAuthSession();

  console.log({ session });
  return (
    <section className="flex flex-col justify-between items-center p-24 min-h-screen">
      Dashboard
    </section>
  );
}
