"use client";

import { useState } from "react";
import { signIn } from "next-auth/react";
import toast from "react-hot-toast";
import { useRouter } from "next/navigation";

export default function Form() {
  const [isLoading, setIsLoading] = useState(false);
  const [errors, setErrors] = useState(null);
  const router = useRouter();

  return (
    <>
      {errors ? (
        <div
          className="p-4 mb-4 text-sm text-red-800 bg-red-50 rounded-lg dark:text-red-400 dark:bg-gray-800"
          role="alert"
        >
          <span className="font-medium">Error alert! </span>
          {errors}
        </div>
      ) : null}
      <form
        onSubmit={(e) => {
          e.preventDefault();
          setIsLoading(true);
          signIn("credentials", {
            redirect: false,
            email: e.currentTarget.email.value,
            password: e.currentTarget.password.value,
            // @ts-ignore
          }).then(({ error }) => {
            if (error) {
              setIsLoading(false);
              // setErrors(error);
              toast.error(error);
            } else {
              router.push("/dashboard");
            }
          });
        }}
        className="space-y-4 md:space-y-6"
      >
        <div>
          <label
            htmlFor="email"
            className="block mb-2 text-sm font-medium text-gray-900 dark:text-white"
          >
            Your email
          </label>
          <input
            type="email"
            id="email"
            name="email"
            className="block p-2.5 w-full text-gray-900 bg-gray-50 rounded-lg border border-gray-300 sm:text-sm dark:placeholder-gray-400 dark:text-white dark:bg-gray-700 dark:border-gray-600 focus:border-blue-600 focus:ring-blue-600 dark:focus:border-blue-500 dark:focus:ring-blue-500"
            placeholder="name@company.com"
            required
          />
        </div>
        <div>
          <label
            htmlFor="password"
            className="block mb-2 text-sm font-medium text-gray-900 dark:text-white"
          >
            Password
          </label>
          <input
            type="password"
            id="password"
            name="password"
            placeholder="••••••••"
            className="block p-2.5 w-full text-gray-900 bg-gray-50 rounded-lg border border-gray-300 sm:text-sm dark:placeholder-gray-400 dark:text-white dark:bg-gray-700 dark:border-gray-600 focus:border-blue-600 focus:ring-blue-600 dark:focus:border-blue-500 dark:focus:ring-blue-500"
            required
          />
        </div>
        <button
          type="submit"
          className="flex justify-center py-2.5 px-5 w-full text-sm font-medium text-center text-white bg-blue-600 rounded-lg dark:bg-blue-600 hover:bg-blue-700 focus:ring-4 focus:ring-blue-300 focus:outline-none disabled:bg-blue-700 dark:hover:bg-blue-700 dark:focus:ring-blue-800"
        >
          {!isLoading ? (
            "Sign in"
          ) : (
            <svg
              className="mr-3 -ml-1 w-5 h-5 text-white animate-spin"
              xmlns="http://www.w3.org/2000/svg"
              fill="none"
              viewBox="0 0 24 24"
            >
              <circle
                className="opacity-25"
                cx="12"
                cy="12"
                r="10"
                stroke="currentColor"
                stroke-width="4"
              ></circle>
              <path
                className="opacity-75"
                fill="currentColor"
                d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
              ></path>
            </svg>
          )}
        </button>
      </form>
    </>
  );
}
