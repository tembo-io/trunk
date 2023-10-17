export const truncate = (str: String, n = 100) => {
  return str.length > n ? str.slice(0, n - 1) + '...' : str;
};
