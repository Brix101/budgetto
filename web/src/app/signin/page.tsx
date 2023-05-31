import Link from "next/link";
import Form from "@/components/Form";

export default function Page() {
  return (
    <>
      <section className="dark:bg-gray-900 bg-slate-200">
        <div className="flex flex-col justify-center items-center py-8 px-6 mx-auto md:h-screen lg:py-0">
          <Link href="/">
            <h2 className="mb-8 text-3xl font-extrabold tracking-tight leading-tight text-center md:text-4xl lg:mb-16 dark:text-white hover:text-gray-900">
              BUDGETTO
            </h2>
          </Link>

          <div className="w-full bg-white rounded-lg shadow sm:max-w-md md:mt-0 xl:p-0 dark:bg-gray-800 dark:border dark:border-gray-700">
            <div className="p-6 space-y-4 sm:p-8 md:space-y-6">
              <h1 className="text-xl font-bold tracking-tight leading-tight text-gray-900 md:text-2xl dark:text-white">
                Sign in to your account
              </h1>
              <Form />
            </div>
          </div>
        </div>
      </section>
    </>
  );
}
