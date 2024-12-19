"use client";

import { api } from "~/utils/trpc/react";

export function Greeter() {
  const greetQuery = api.greeting.useQuery();
  return (
    <div>
      Hello, world!
      {greetQuery.data ? greetQuery.data : "Loading..."}
    </div>
  );
}
