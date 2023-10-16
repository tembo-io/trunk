import Head from 'next/head';
import '@/globals.scss';
import type { AppProps } from 'next/app';
import { Inter, IBM_Plex_Mono } from 'next/font/google';
import '../styles/markdown.css';
import Footer from '@/Components/Footer';
const inter = Inter({
  subsets: ['latin'],
  weight: ['400', '500', '600', '700'],
  variable: '--font-inter',
});
export const plex_mono = IBM_Plex_Mono({
  weight: ['700', '400'],
  subsets: ['latin'],
  variable: '--font-plex',
});
export default function App({ Component, pageProps }: AppProps) {
  return (
    <>
      <Head>
        <link rel="shortcut icon" href="/favicon.ico" />
        <script
          defer
          data-domain="pgt.dev"
          src="https://plausible.io/js/script.js"></script>
      </Head>
      <main className={`${plex_mono.variable} ${inter.variable}`}>
        <Component {...pageProps} />
        <Footer />
      </main>
    </>
  );
}
