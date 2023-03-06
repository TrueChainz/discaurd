import { Html, Head, Main, NextScript } from "next/document";

export default function Document() {
  return (
    <Html>
      <Head />
      <body data-theme="dark">
        <Main />
        <NextScript />
        <div id="modal" />
      </body>
    </Html>
  );
}
