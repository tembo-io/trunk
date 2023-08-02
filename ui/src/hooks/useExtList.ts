import useSWR from "swr";

const fetcher = (...args: [RequestInfo, RequestInit]): Promise<any> => fetch(...args).then((res) => res.json());

export default function useExtList() {
  const { data, error, isLoading } = useSWR(`${process.env.NEXT_PUBLIC_API_BASE_URL}/extensions/all`, fetcher);

  return {
    extensions: data,
    isLoading,
    isError: error,
  };
}
