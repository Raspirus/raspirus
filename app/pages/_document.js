import { Html, Head, Main, NextScript } from 'next/document'

export default function Document() {
  return (
    <Html lang="en">
      <Head>
        <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/4.7.0/css/font-awesome.min.css" />
        <script defer src="https://unpkg.com/alpinejs@3.11.1/dist/cdn.min.js"></script>
      </Head>
        <body>
          <Main />
          <NextScript />
        </body>
    </Html>
  )
}
