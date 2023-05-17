"use server";

import { baseApi } from "@/constant/server";
import { redirect } from "next/navigation";
import { cookies } from "next/headers";

export async function signInAction(formData: FormData) {
  const cookieStore = cookies();

  const email = formData.get("email");
  const password = formData.get("password");

  console.log({ email, password });
  const res = await fetch(`${baseApi}/users/signin`, {
    headers: {
      Accept: "application/json",
      "Content-Type": "application/json",
    },
    method: "POST",
    body: JSON.stringify({
      email,
      password,
    }),
  });

  const cookie = res.headers.get("set-cookie");
  if (cookie) {
    // Split the string using ";"
    const parts: string[] = cookie.split(";");

    // Create an empty JSON object
    const jsonObject: { [key: string]: string | boolean } = {};

    // Iterate through the parts and add key-value pairs to the JSON object
    for (let i = 0; i < parts.length; i++) {
      const keyValue: string[] = parts[i].trim().split("=");
      const key: string = keyValue[0].trim();
      const value: string | boolean = keyValue[1] ? keyValue[1].trim() : true;

      // Convert the key to lowercase with the first letter in lowercase
      const lowerCaseKey: string = key.charAt(0).toLowerCase() + key.slice(1);

      if (lowerCaseKey === "x-refresh") {
        jsonObject["value"] = value;
      } else {
        jsonObject[lowerCaseKey] = value;
      }
    }

    cookieStore.set({
      name: "x-refresh",
      ...jsonObject,
    });
  }

  const body = await res.json();
  cookieStore.set({
    name: "authorization",
    value: body.accessToken,
    httpOnly: true,
    path: "/",
  });

  if (res.ok) {
    redirect("/dashboard");
  }
}
