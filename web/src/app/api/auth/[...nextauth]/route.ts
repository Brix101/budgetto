import { baseApi } from "@/constant/server";
import NextAuth, { type NextAuthOptions } from "next-auth";
import CredentialsProvider from "next-auth/providers/credentials";
import { env } from "process";

export const authOptions: NextAuthOptions = {
  callbacks: {
    async jwt({ token, user }) {
      if (user) {
        return { user };
      }

      const res = await fetch(`${baseApi}/auth/whoami`, {
        headers: {
          Authorization: `Bearer ${token.user.accessToken}`,
          cookie: `x-refresh=${token.user.refreshToken}`,
        },
      });

      const xAccessToken = res.headers.get("x-access-token");
      const accessToken = xAccessToken ?? token.user.accessToken;

      const body = await res.json();

      return {
        ...token,
        user: {
          ...token.user,
          ...body.user,
          accessToken,
        },
      };
    },
    session: ({ session, token }) => {
      return {
        ...session,
        user: {
          ...token.user,
        },
      };
    },
  },
  secret: env.NEXTAUTH_SECRET,
  session: {
    strategy: "jwt",
  },
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
          const res = await fetch(`${baseApi}/auth/sign-in`, {
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

          const xRefresh = res.headers.get("set-cookie");
          const body = await res.json();
          // Create an empty JSON object
          const jsonObject: { [key: string]: string | boolean } = {};

          if (xRefresh) {
            // Split the string using ";"
            const parts: string[] = xRefresh.split(";");

            // Iterate through the parts and add key-value pairs to the JSON object
            for (let i = 0; i < parts.length; i++) {
              const keyValue: string[] = parts[i].trim().split("=");
              const key: string = keyValue[0].trim();
              const value: string | boolean = keyValue[1]
                ? keyValue[1].trim()
                : true;

              // Convert the key to lowercase with the first letter in lowercase
              const lowerCaseKey: string =
                key.charAt(0).toLowerCase() + key.slice(1);

              if (lowerCaseKey === "x-refresh") {
                jsonObject["value"] = value;
              } else {
                jsonObject[lowerCaseKey] = value;
              }
            }
          }

          if (res.ok) {
            const user = {
              ...body.user,
              accessToken: body.accessToken,
              refreshToken: jsonObject["value"] ?? "",
            };
            return user;
          } else {
            throw body.errors;
          }
        } catch (error: any) {
          console.error(`NextAuth authorize error: ${error.message}`);
          throw error;
        }
      },
    }),
  ],
  events: {
    async signOut({ token }) {
      await fetch(`${baseApi}/auth/sign-out`, {
        method: "POST",
        headers: {
          Authorization: `Bearer ${token.user.accessToken}`,
          cookie: `x-refresh=${token.user.refreshToken}`,
        },
      });
    },
  },
};

const handler = NextAuth(authOptions);

export { handler as GET, handler as POST };
