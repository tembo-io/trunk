const fetchExtensions = async () => {
  const apiRes = await fetch(`${process.env.NEXT_PUBLIC_API_BASE_URL}/extensions/all`);
  if (!apiRes.ok) {
    throw new Error("Error fetching all extensions");
  }
  return apiRes.json();
};

export default fetchExtensions;
