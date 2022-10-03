/** @type {import('next').NextConfig} */
const nextConfig = {
  reactStrictMode: true,
  swcMinify: true,
  images: {
    domains: [
      "raw.githubusercontent.com",
      "www.arkhadian.com",
      "s.imversed.com",
      "defitech-logo.s3.amazonaws.com",
      "app.stateset.zone",
    ],
    formats: ["image/avif", "image/webp"],
  },
};

module.exports = nextConfig;
