const fetchExtensions = async () => {
  const apiRes = await fetch(`https://registry.pgtrunk.io/extensions/all`);
  if (!apiRes.ok) {
    throw new Error("Error fetching all extensions");
  }
  return apiRes.json();
};

export default fetchExtensions;
