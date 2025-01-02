"use client";

import type { SignInResponse } from "next-auth/react";
import { useRouter } from "next/navigation";
import { signIn } from "next-auth/react";

import type { SignInDto } from "@budgetto/schema";
import { signInSchema } from "@budgetto/schema";
import { Button } from "@budgetto/ui/button";
import {
  Form,
  FormControl,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
  useForm,
} from "@budgetto/ui/form";
import { Input } from "@budgetto/ui/input";

export function SignInForm() {
  const router = useRouter();

  const form = useForm({
    schema: signInSchema,
    defaultValues: {
      email: "",
      password: "",
    },
  });

  function onSubmit(values: SignInDto) {
    signIn("credentials", {
      ...values,
      redirect: false,
    })
      .then((res: SignInResponse | undefined) => {
        console.log(res);
        if (res?.ok) {
          if (res.url) {
            router.push(res.url);
          } else {
            router.push("/");
          }
        }
      })
      .catch((error) => {
        console.error(error);
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
        <Button type="submit">Submit</Button>
      </form>
    </Form>
  );
}
