/** @type {import('next').NextConfig} */
const nextConfig = {
  reactStrictMode: true,
  images: {
    remotePatterns: [
      {
        protocol: "https",
        hostname: "**.images.clerk.dev/**",
      },
      {
        protocol: "https",
        hostname: "**.pgtrunk.io/**",
      },
      {
        protocol: "https",
        hostname: "**.amplifyapp.com/**",
      },
    ],
  },
};

module.exports = nextConfig;
