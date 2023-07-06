import Head from "next/head";
import { ClerkProvider, ClerkLoaded } from "@clerk/nextjs";
import type { AppProps } from "next/app";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
function MyApp({ Component, pageProps }: AppProps) {
  const queryClient = new QueryClient();

  return (
    <>
      <Head>
        <link rel="shortcut icon" href="/favicon.ico" />
      </Head>
      <ClerkProvider {...pageProps}>
        <ClerkLoaded>
          <QueryClientProvider client={queryClient}>
            <Component {...pageProps} />
          </QueryClientProvider>
        </ClerkLoaded>
      </ClerkProvider>
    </>
  );
}

export default MyApp;
