import { baseApi } from "@/constant/server";
import {
  DefaultSession,
  getServerSession,
  type NextAuthOptions,
} from "next-auth";
import CredentialsProvider from "next-auth/providers/credentials";
import { env } from "process";

/**
 * Module augmentation for `next-auth` types. Allows us to add custom properties to the `session`
 * object and keep type safety.
 *
 * @see https://next-auth.js.org/getting-started/typescript#module-augmentation
 */
declare module "next-auth" {
  /**
   * Returned by `useSession`, `getSession` and received as a prop on the `SessionProvider` React Context
   */
  interface Session {
    user: {
      /** The user's postal address. */
      accessToken: string;
      refreshToken: string;
    } & DefaultSession["user"];
  }

  interface User {
    /** The user's postal address. */
    accessToken: string;
    refreshToken: string;
  }
}
/**
 * Options for NextAuth.js used to configure adapters, providers, callbacks, etc.
 *
 * @see https://next-auth.js.org/configuration/options
 */
export const authOptions: NextAuthOptions = {
  callbacks: {
    async jwt({ token, user }) {
      if (user) {
        return user;
      }

      const res = await fetch(`${baseApi}/auth/whoami`, {
        headers: {
          Authorization: `Bearer ${token.accessToken}`,
          cookie: `x-refresh=${token.refreshToken}`,
        },
      });

      const xAccessToken = res.headers.get("x-access-token");
      const accessToken = xAccessToken ?? token.accessToken;

      const body = await res.json();

      return {
        ...token,
        ...body.user,
        accessToken,
      };
    },
    session: ({ session, token }) => ({
      ...session,
      user: {
        ...token,
      },
    }),
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
            return {
              ...body.user,
              accessToken: body.accessToken,
              refreshToken: jsonObject["value"] ?? "",
            };
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
          Authorization: `Bearer ${token.accessToken}`,
          cookie: `x-refresh=${token.refreshToken}`,
        },
      });
    },
  },
};
/**
 * Wrapper for `getServerSession` so that you don't need to import the `authOptions` in every file.
 *
 * @see https://next-auth.js.org/configuration/nextjs
 */
export const getServerAuthSession = () => getServerSession(authOptions);
