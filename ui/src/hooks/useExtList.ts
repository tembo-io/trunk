import useSWR from "swr";

const fetcher = (...args: [RequestInfo, RequestInit]): Promise<any> => fetch(...args).then((res) => res.json());

export default function useExtList() {
  const { data, error, isLoading } = useSWR("https://registry.pgtrunk.io/extensions/all", fetcher);

  return {
    extensions: data,
    isLoading,
    isError: error,
  };
}
