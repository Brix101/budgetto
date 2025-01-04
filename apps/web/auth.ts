import { cache } from "react";
import { cookies, headers } from "next/headers";
import NextAuth, { DefaultSession, NextAuthConfig } from "next-auth";
import CredentialsProvider from "next-auth/providers/credentials";
import { ZodErrorMap } from "zod";

import { SignInDto } from "@budgetto/schema/auth";

import { env } from "~/env";

const privateRoutes = ["/dashboard"];

declare module "next-auth" {
  interface User {
    id?: string;
    email?: string | null;
    accessToken: string;
    refreshToken: string;
    exp: number;
    role: string;
    message?: string;
    errors?: ZodErrorMap;
  }

  interface Session {
    user: User & DefaultSession["user"];
    expires: string;
    error: string;
  }
}

// @ts-ignore
async function refreshAccessToken(token) {
  // this is our refresh token method
  console.log("Now refreshing the expired token...");
  try {
    const res = await fetch(`${env.NEXT_PUBLIC_BASE_URL}/api/auth/refresh`, {
      method: "POST",
      headers: await headers(),
      body: JSON.stringify({ userId: token.userId }),
    });

    const { success, data } = await res.json();

    if (!success) {
      console.log("The token could not be refreshed!");
      throw data;
    }

    console.log("The token has been refreshed successfully.");

    // get some data from the new access token such as exp (expiration time)
    const decodedAccessToken = JSON.parse(
      Buffer.from(data.accessToken.split(".")[1], "base64").toString(),
    );

    return {
      ...token,
      accessToken: data.accessToken,
      refreshToken: data.refreshToken ?? token.refreshToken,
      accessTokenExpires: decodedAccessToken.exp * 1000,
      error: "",
    };
  } catch (error) {
    console.log(error);

    // return an error if somethings goes wrong
    return {
      ...token,
      error: "RefreshAccessTokenError", // attention!
    };
  }
}

const authConfig = {
  trustHost: true,
  theme: {
    logo: "https://next-auth.js.org/img/logo/logo-sm.png",
  },
  providers: [
    CredentialsProvider({
      credentials: {
        email: {
          label: "email",
          type: "email",
          placeholder: "jsmith@example.com",
        },
        password: {
          label: "password",
          type: "password",
        },
      },
      async authorize(credentials, request) {
        const cookieStore = await cookies();

        const payload = {
          email: credentials.email,
          password: credentials.password,
        } as SignInDto;

        // external api for users to log in, change it with your own endpoint
        const res = await fetch(`${env.API_BASE_URL}/api/auth/sign-in`, {
          method: "POST",
          headers: {
            "Content-Type": "application/json",
            "Access-Control-Allow-Origin": "*",
          },
          body: JSON.stringify(payload),
        });

        const data = await res.json();

        if (!res.ok) {
          return data;
        }

        if (res.ok && data) {
          const prefix = env.NODE_ENV === "development" ? "__Dev-" : "";

          // we set http only cookie here to store refresh token information as we will not append it to our session to avoid maximum size warning for the session cookie (4096 bytes)
          cookieStore.set({
            name: `${prefix}budgetto.refresh-token`,
            value: data.refreshToken,
            httpOnly: true,
            sameSite: "strict",
            secure: true,
          } as any);

          return {
            ...data,
            id: data.user.id,
          };
        }

        return null;
      },
    }),
  ],
  callbacks: {
    async jwt({ token, user, account }) {
      if (account && user) {
        token.id = user.id;
        token.accessToken = user.accessToken;
        token.refreshToken = user.refreshToken;
        token.role = "Unknown";

        const dataToken = user.accessToken.split(".")[1] || "";

        const decodedAccessToken = JSON.parse(
          Buffer.from(dataToken, "base64").toString(),
        );

        if (decodedAccessToken) {
          token.userId = decodedAccessToken.sub as string;
          token.accessTokenExpires = decodedAccessToken.exp * 1000;
        }
      }

      // if our access token has not expired yet, return all information except the refresh token
      if (
        (token.accessTokenExpires &&
          Date.now() < Number(token.accessTokenExpires)) ||
        token.error == "RefreshAccessTokenError"
      ) {
        const { refreshToken, ...rest } = token;
        return rest;
      }

      return await refreshAccessToken(token);
    },
    async session({ session, token, user }) {
      return {
        ...session,
        user: {
          ...session.user,
          id: token.id as string,
          name: token.name!,
          email: token.email!,
          accessToken: token.accessToken as string,
          accessTokenExpires: token.accessTokenExpires as number,
        },
        error: token.error,
      };
    },
    authorized({ request, auth }) {
      const { pathname } = request.nextUrl;

      // get the route name from the url such as "/about"
      const searchTerm = request.nextUrl.pathname
        .split("/")
        .slice(0, 2)
        .join("/");

      // if the private routes array includes the search term, we ask authorization here and forward any unauthorized users to the login page
      if (privateRoutes.includes(searchTerm)) {
        console.log(
          `${auth ? "Can" : "Cannot"} access private route ${searchTerm}`,
        );
        return !!auth;
        // if the pathname starts with one of the routes below and the user is already logged in, forward the user to the home page
      } else if (
        pathname.startsWith("/sign") ||
        pathname.startsWith("/forgot-password")
      ) {
        const isLoggedIn = !!auth;

        if (isLoggedIn) {
          return Response.redirect(new URL("/", request.nextUrl));
        }

        return true;
      }

      return true;
    },
    redirect({ url, baseUrl }) {
      const callBackUrl = new URL(url).searchParams.get("callbackUrl");

      return callBackUrl ?? baseUrl;
    },
    signIn({ user }) {
      if (user?.errors) {
        throw new Error(user.message, {
          cause: user.errors,
        });
      }

      return true;
    },
  },
  // this is required
  secret: env.AUTH_SECRET,
  // our custom login page
  pages: {
    signIn: "/sign-in",
  },
  debug: env.NODE_ENV === "development",
} satisfies NextAuthConfig;

const { handlers, auth: defaultAuth, signIn, signOut } = NextAuth(authConfig);

/**
 * This is the main way to get session data for your RSCs.
 * This will de-duplicate all calls to next-auth's default `auth()` function and only call it once per request
 */
// const auth = cache(defaultAuth);

export { handlers, defaultAuth as auth, signIn, signOut };
