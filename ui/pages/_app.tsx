import { ClerkProvider, ClerkLoaded } from "@clerk/nextjs";
import type { AppProps } from "next/app";
import { useQuery, useMutation, useQueryClient, QueryClient, QueryClientProvider } from "@tanstack/react-query";

function MyApp({ Component, pageProps }: AppProps) {
  const queryClient = new QueryClient();

  return (
    <ClerkProvider {...pageProps}>
      <ClerkLoaded>
        <QueryClientProvider client={queryClient}>
          <Component {...pageProps} />
        </QueryClientProvider>
      </ClerkLoaded>
    </ClerkProvider>
  );
}

export default MyApp;
