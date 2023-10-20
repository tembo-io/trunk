import useSWR from 'swr';

const fetcher = (...args: [RequestInfo, RequestInit]): Promise<any> =>
  fetch(...args).then((res) => res.json());
const REGISTRY_URL =
  process.env.NEXT_PUBLIC_API_BASE_URL || 'https://registry.pgtrunk.io';

export default function useExtList() {
  const { data, error, isLoading } = useSWR(
    `${REGISTRY_URL}/extensions/all`,
    fetcher
  );

  return {
    extensions: data,
    isLoading,
    isError: error,
  };
}
