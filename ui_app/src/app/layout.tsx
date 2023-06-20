import "./globals.scss";
import cx from "classNames";
import { Inter, IBM_Plex_Mono } from "next/font/google";
import classNames from "classnames";

const inter = Inter({ subsets: ["latin"], weight: ["500", "700"], variable: "--font-inter" });
const plex_mono = IBM_Plex_Mono({ weight: "700", subsets: ["latin"], variable: "--font-plex" });

export const metadata = {
  title: "Trunk",
  description: "Postgres extension registry",
};

export default function RootLayout({ children }: { children: React.ReactNode }) {
  return (
    <html lang="en" className={`${inter.variable} ${plex_mono.variable}`}>
      <body>{children}</body>
    </html>
  );
}
