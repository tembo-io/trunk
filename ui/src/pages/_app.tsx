import "@/globals.scss";
import type { AppProps } from "next/app";
import { Inter, IBM_Plex_Mono } from "next/font/google";
import "../styles/markdown.css";
const inter = Inter({ subsets: ["latin"], weight: ["400", "500", "600", "700"], variable: "--font-inter" });
export const plex_mono = IBM_Plex_Mono({ weight: ["700", "400"], subsets: ["latin"], variable: "--font-plex" });

export default function App({ Component, pageProps }: AppProps) {
  return (
    <main className={`${plex_mono.variable} ${inter.variable}`}>
      <Component {...pageProps} />
    </main>
  );
}
