import "@/styles/globals.css";
import type { AppProps } from "next/app";
import { Inter, IBM_Plex_Mono } from "next/font/google";

const inter = Inter({ subsets: ["latin"], weight: ["500", "700"], variable: "--font-inter" });
export const plex_mono = IBM_Plex_Mono({ weight: "700", subsets: ["latin"], variable: "--font-plex" });

export default function App({ Component, pageProps }: AppProps) {
  console.log("APP");

  return (
    <main className={`${plex_mono.variable} ${inter.variable}`}>
      <Component {...pageProps} />
    </main>
  );
}
