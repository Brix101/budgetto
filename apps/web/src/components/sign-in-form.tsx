"use client";

import React from "react";
import { useRouter } from "next/navigation";
import { signIn } from "next-auth/react";

import type { SignInDto } from "@budgetto/schema/auth";
import { signInSchema } from "@budgetto/schema/auth";

import { Button } from "~/components/ui/button";
import {
  Form,
  FormControl,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
  useForm,
} from "~/components/ui/form";
import { Input } from "~/components/ui/input";

export function SignInForm() {
  const router = useRouter();
  const [isPending, startTransitionz] = React.useTransition();

  const form = useForm({
    schema: signInSchema,
    defaultValues: {
      email: "",
      password: "",
    },
  });

  function onSubmit(values: SignInDto) {
    startTransitionz(async () => {
      try {
        const result = await signIn("credentials", {
          ...values,
          redirect: false,
        });

        if (result?.error) {
          form.setError(
            "email",
            {
              type: "manual",
              message: "The email or password is incorrect.",
            },
            {
              shouldFocus: true,
            },
          );
          form.setError("password", { message: "" });
        } else {
          if (result?.url) {
            router.push(result.url);
          } else {
            router.push("/dashboard");
          }
        }
      } catch (e) {
        console.log(e);
      }
    });
  }

  return (
    <Form {...form}>
      <form onSubmit={form.handleSubmit(onSubmit)} className="space-y-8">
        <FormField
          control={form.control}
          name="email"
          render={({ field }) => (
            <FormItem>
              <FormLabel>Email</FormLabel>
              <FormControl>
                <Input placeholder="john.doe@example.com" {...field} />
              </FormControl>
              {/* <FormDescription>
                This is your public display name.
              </FormDescription> */}
              <FormMessage />
            </FormItem>
          )}
        />
        <FormField
          control={form.control}
          name="password"
          render={({ field }) => (
            <FormItem>
              <FormLabel>Password</FormLabel>
              <FormControl>
                <Input placeholder="**********" {...field} />
              </FormControl>
              {/* <FormDescription>
                This is your public display name.
              </FormDescription> */}
              <FormMessage />
            </FormItem>
          )}
        />
        <Button type="submit" disabled={isPending} className="w-full">
          {isPending ? "Signing in..." : "Sign in"}
        </Button>
      </form>
    </Form>
  );
}
