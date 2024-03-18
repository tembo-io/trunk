import { Html, Head, Main, NextScript } from 'next/document';

export default function Document() {
  return (
    <Html lang="en">
      <Head>
        <meta name="twitter:card" content="summary_large_image" />
        <meta
          name="twitter:description"
          content="Trunk is an open-source package installer and registry for PostgreSQL extensions."
        />
        <meta name="twitter:site" content="@tembo_io" />
        <meta name="twitter:title" content="Trunk" />
        <meta name="twitter:image" content="https://pgt.dev/og-image.png" />
        <meta property="og:image" content="https://pgt.dev/og-image.png" />
        <meta property="og:title" content="Trunk" />
        <meta
          property="og:description"
          content="Trunk is an open-source package installer and registry for PostgreSQL extensions."
        />
        <meta property="og:type" content="website" />
        <meta
          name="description"
          content="Trunk is an open-source package installer and registry for PostgreSQL extensions."
        />
      </Head>
      <body>
        <Main />
        <NextScript />
      </body>
    </Html>
  );
}
