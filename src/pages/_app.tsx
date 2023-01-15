import type { AppProps } from "next/app";
import { SessionProvider } from "next-auth/react";

import "../style.css";
import "../App.css";

// This default export is required in a new `pages/_app.js` file.
export default function MyApp({
  Component,
  pageProps: { session, ...pageProps },
}: AppProps) {
  return (
    <SessionProvider session={session}>
      <Component {...pageProps} />
    </SessionProvider>
  );
}
