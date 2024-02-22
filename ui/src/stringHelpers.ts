export const truncate = (str: String, n = 100) => {
  if (!str) return '';
  return str.length > n ? str.slice(0, n - 1) + '...' : str;
};
