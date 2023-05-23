import { Html, Head, Main, NextScript } from 'next/document';

/**
 * This is a special file defined by Next.js that loads the specific setup of the page. 
 * It basically wraps all other pages or components.
 * Warning: This is loaded for each component, if you wish to load something a single time only use the _app.js file.
 * @returns HTML structure that wraps pages
 */
export default function Document() {
  return (
    <Html lang="en">
      <Head>
      </Head>
        <body>
          <Main />
          <NextScript />
        </body>
    </Html>
  )
}
