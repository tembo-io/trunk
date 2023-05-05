/** @type {import('next').NextConfig} */
const nextConfig = {
  reactStrictMode: true,
  images: {
    remotePatterns: [
      {
        hostname: "images.clerk.dev",
      },
    ],
  },
};

module.exports = nextConfig;
