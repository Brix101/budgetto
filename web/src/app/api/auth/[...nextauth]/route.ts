import { baseApi } from "@/constant/server";
import NextAuth, { type NextAuthOptions } from "next-auth";
import CredentialsProvider from "next-auth/providers/credentials";

export const authOptions: NextAuthOptions = {
  providers: [
    CredentialsProvider({
      credentials: {
        email: { label: "Email", type: "email" },
        password: { label: "Password", type: "password" },
      },
      async authorize(credentials) {
        try {
          const { email, password } = credentials ?? {};
          if (!email || !password) {
            throw new Error("Missing username or password");
          }
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
          console.log(res.headers.get("set-cookie"));
          if (res.ok) {
            const body = await res.json();
            return body;
          }
        } catch (error: any) {
          console.error(`NextAuth authorize error: ${error.message}`);
          throw error;
        }
      },
    }),
  ],
};

const handler = NextAuth(authOptions);

export { handler as GET, handler as POST };
