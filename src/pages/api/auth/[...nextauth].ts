import NextAuth from "next-auth";
import CredentialsProvider from "next-auth/providers/credentials";
import { authenticate } from "../../../lib/services";

interface User {
  header: number;
}

export default NextAuth({
  callbacks: {
    signIn: async (params) => {
      if (params.user) return true;
      return false;
    },
    jwt: async ({ token, user }) => {
      console.log("token callback jwt: ", token);
      console.log("user in jwt: ");
      if (user) {
        token.username = user.username;
      }
      return token;
    },
    session: async ({ session, token }) => {
      return { ...session, user: { username: token.username } };
    },
  },
  session: {
    strategy: "jwt",
  },
  providers: [
    // GithubProvider({
    //   clientId: process.env.GITHUB_ID,
    //   clientSecret: process.env.GITHUB_SECRET,
    // }),
    CredentialsProvider({
      async authorize(credentials, req) {
        try {
          const authenticated = await authenticate(credentials);
          console.log({
            id: authenticated.id,
            username: authenticated.username,
          });
          return {
            id: authenticated.id,
            username: authenticated.username,
          };
        } catch (err) {
          throw new Error(err);
        }
      },
      credentials: {
        username: { label: "Username", type: "text" },
        password: { label: "Password", type: "password" },
      },
    }),
  ],
  pages: { signIn: "/auth/login", newUser: "/auth/register" },
});
