import type { OAuthConfig, OAuthUserConfig } from 'next-auth/providers';

interface KakaoProfile {
  id: number;
  connected_at: string;
  kakao_account?: {
    email?: string;
    profile?: {
      nickname?: string;
      profile_image_url?: string;
    };
  };
}

export default function KakaoProvider(
  config: OAuthUserConfig<KakaoProfile>,
): OAuthConfig<KakaoProfile> {
  return ({
    id: 'kakao',
    name: 'Kakao',
    type: 'oauth',
    authorization: {
      url: 'https://kauth.kakao.com/oauth/authorize',
      params: { scope: 'account_email profile_nickname' },
    },
    token: 'https://kauth.kakao.com/oauth/token',
    userinfo: 'https://kapi.kakao.com/v2/user/me',
    profile(profile) {
      return {
        id: String(profile.id),
        name: profile.kakao_account?.profile?.nickname,
        email: profile.kakao_account?.email,
        image: profile.kakao_account?.profile?.profile_image_url,
      };
    },
    ...config,
  }) as OAuthConfig<KakaoProfile>;
}
