import { User, Session } from "next-auth";
import { JWT } from "next-auth/jwt";

declare module "next-auth" {
  interface User {
    username?: string;
  }
  interface Session {
    user: {
      username?: string;
    };
  }
}

declare module "next-auth/jwt" {
  interface JWT {
    username?: string;
  }
}
