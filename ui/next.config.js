/** @type {import('next').NextConfig} */
const nextConfig = {
  reactStrictMode: true,
  /* Do not use this for server-only env vars
   * see: https://nextjs.org/docs/pages/api-reference/next-config-js/env
   * use this approach instead: https://docs.aws.amazon.com/amplify/latest/userguide/ssr-environment-variables.html
   * also see: https://nextjs.org/docs/pages/building-your-application/configuring/environment-variables
   */
  env: {},
};

module.exports = nextConfig;
