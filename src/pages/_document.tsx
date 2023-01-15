import { useAutoAnimate } from "@formkit/auto-animate/react";
import { Html, Head, Main, NextScript } from "next/document";
import { LegacyRef } from "react";
import AddFriendModal from "../components/AddFriendModal";

export default function Document() {
  return (
    <Html>
      <Head />
      <body>
        <Main />
        <NextScript />
        <div id="modal" />
      </body>
    </Html>
  );
}
