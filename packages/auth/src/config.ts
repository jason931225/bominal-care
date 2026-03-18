import NextAuth from 'next-auth';
import GoogleProvider from 'next-auth/providers/google';
import CredentialsProvider from 'next-auth/providers/credentials';
import { pool } from '@bominal-senior/db';

type UserRole =
  | 'SENIOR'
  | 'FAMILY'
  | 'CAREGIVER_APPLICANT'
  | 'CAREGIVER_APPROVED'
  | 'PROVIDER_ADMIN'
  | 'PROVIDER_STAFF'
  | 'MEDICAL_STAFF'
  | 'GOVERNMENT_REVIEWER'
  | 'PARTNER_OPERATOR'
  | 'PLATFORM_ADMIN';

type KycLevel =
  | 'NONE'
  | 'EMAIL_VERIFIED'
  | 'PHONE_VERIFIED'
  | 'IDENTITY_VERIFIED'
  | 'FULL_VERIFIED';

import KakaoProvider from './providers/kakao';
import NaverProvider from './providers/naver';

export const { handlers, auth, signIn, signOut } = NextAuth({
  session: {
    strategy: 'jwt',
  },
  providers: [
    CredentialsProvider({
      id: 'demo-login',
      name: 'Demo Login',
      credentials: {
        email: { label: 'Email', type: 'email' },
      },
      async authorize(credentials) {
        const email = credentials?.email as string | undefined;
        if (!email) return null;

        const result = await pool.query(
          'SELECT id, email, name, role, kyc_level, is_active FROM users WHERE email = $1 AND is_active = true',
          [email],
        );
        const user = result.rows[0];
        if (!user) return null;

        return {
          id: user.id,
          email: user.email,
          name: user.name,
          role: user.role,
          kycLevel: user.kyc_level,
        };
      },
    }),
    KakaoProvider({
      clientId: process.env.KAKAO_CLIENT_ID ?? '',
      clientSecret: process.env.KAKAO_CLIENT_SECRET ?? '',
    }),
    NaverProvider({
      clientId: process.env.NAVER_CLIENT_ID ?? '',
      clientSecret: process.env.NAVER_CLIENT_SECRET ?? '',
    }),
    GoogleProvider({
      clientId: process.env.GOOGLE_CLIENT_ID ?? '',
      clientSecret: process.env.GOOGLE_CLIENT_SECRET ?? '',
    }),
  ],
  callbacks: {
    async jwt({ token, user }) {
      if (user) {
        token.id = user.id;
        // Use role/kycLevel from credentials authorize or fetch from DB
        const authUser = user as { role?: string; kycLevel?: string };
        if (authUser.role) {
          token.role = authUser.role;
          token.kycLevel = authUser.kycLevel ?? 'NONE';
        } else {
          // OAuth flow — look up role from DB
          const result = await pool.query(
            'SELECT role, kyc_level FROM users WHERE id = $1',
            [user.id],
          );
          const dbUser = result.rows[0];
          token.role = (dbUser?.role ?? 'SENIOR') as UserRole;
          token.kycLevel = (dbUser?.kyc_level ?? 'NONE') as KycLevel;
        }
      }
      return token;
    },
    async session({ session, token }) {
      return {
        ...session,
        user: {
          ...session.user,
          id: token.id as string,
          role: token.role as UserRole,
          kycLevel: token.kycLevel as KycLevel,
          tenantId: token.tenantId as string | undefined,
        },
      };
    },
  },
  pages: {
    signIn: '/auth/signin',
    error: '/auth/error',
  },
});
