import type { NextConfig } from 'next';

const nextConfig: NextConfig = {
  transpilePackages: ['@bominal-senior/ui', '@bominal-senior/auth', '@bominal-senior/types'],
};

export default nextConfig;
