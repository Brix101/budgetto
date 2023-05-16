import { baseApi } from "@/constant/server";
import Image from "next/image";
import Link from "next/link";

async function getData() {
  const res = await fetch(`${baseApi}/ping`);
  // The return value is *not* serialized
  // You can return Date, Map, Set, etc.

  // Recommendation: handle errors
  if (!res.ok) {
    // This will activate the closest `error.js` Error Boundary
    throw new Error("Failed to fetch data");
  }

  return res.json();
}

export default async function Home() {
  const data = await getData();

  return (
    <main className="flex flex-col justify-between items-center p-24 min-h-screen">
      <div className="z-10 justify-between items-center w-full max-w-5xl font-mono text-sm lg:flex">
        <div className="flex fixed bottom-0 left-0 justify-center items-end w-full h-48 bg-gradient-to-t from-white via-white lg:static lg:w-auto lg:h-auto lg:bg-none dark:from-black dark:via-black">
          <a
            className="flex gap-2 place-items-center p-8 pointer-events-none lg:p-0 lg:pointer-events-auto"
            href="https://vercel.com?utm_source=create-next-app&utm_medium=appdir-template&utm_campaign=create-next-app"
            target="_blank"
            rel="noopener noreferrer"
          >
            By{" "}
            <Image
              src="/vercel.svg"
              alt="Vercel Logo"
              className="dark:invert"
              width={100}
              height={24}
              priority
            />
          </a>
        </div>
        <Link href={"/sign-in"}>
          <p className="flex fixed top-0 left-0 justify-center pt-8 pb-6 w-full bg-gradient-to-b border-b border-gray-300 lg:static lg:p-4 lg:w-auto lg:bg-gray-200 lg:rounded-xl lg:border from-zinc-200 backdrop-blur-2xl lg:dark:bg-zinc-800/30 dark:border-neutral-800 dark:bg-zinc-800/30 dark:from-inherit">
            {/* Get started by editing&nbsp; */}
            {/* <code className="font-mono font-bold">src/app/page.tsx</code> */}
            {data.message} Sign in
          </p>
        </Link>
      </div>

      <div className="flex relative place-items-center before:absolute before:h-[300px] before:w-[480px] before:-translate-x-1/2 before:rounded-full before:bg-gradient-radial before:from-white before:to-transparent before:blur-2xl before:content-[''] after:absolute after:-z-20 after:h-[180px] after:w-[240px] after:translate-x-1/3 after:bg-gradient-conic after:from-sky-200 after:via-blue-200 after:blur-2xl after:content-[''] before:dark:bg-gradient-to-br before:dark:from-transparent before:dark:to-blue-700 before:dark:opacity-10 after:dark:from-sky-900 after:dark:via-[#0141ff] after:dark:opacity-40 before:lg:h-[360px]">
        <Image
          className="relative dark:drop-shadow-[0_0_0.3rem_#ffffff70] dark:invert"
          src="/next.svg"
          alt="Next.js Logo"
          width={180}
          height={37}
          priority
        />
      </div>
    </main>
  );
}
