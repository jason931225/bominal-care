import type { OAuthConfig, OAuthUserConfig } from 'next-auth/providers';

interface NaverProfile {
  resultcode: string;
  message: string;
  response: {
    id: string;
    nickname?: string;
    email?: string;
    profile_image?: string;
    name?: string;
    mobile?: string;
  };
}

export default function NaverProvider(
  config: OAuthUserConfig<NaverProfile>,
): OAuthConfig<NaverProfile> {
  return ({
    id: 'naver',
    name: 'Naver',
    type: 'oauth',
    authorization: {
      url: 'https://nid.naver.com/oauth2.0/authorize',
    },
    token: 'https://nid.naver.com/oauth2.0/token',
    userinfo: 'https://openapi.naver.com/v1/nid/me',
    profile(profile) {
      const resp = profile.response;
      return {
        id: resp.id,
        name: resp.name ?? resp.nickname,
        email: resp.email,
        image: resp.profile_image,
      };
    },
    ...config,
  }) as OAuthConfig<NaverProfile>;
}
