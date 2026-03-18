import { NextResponse } from 'next/server';

export function apiSuccess<T>(
  data: T,
  meta?: { total: number; page: number; limit: number; totalPages: number },
) {
  return NextResponse.json({ success: true, data, error: null, meta: meta ?? null });
}

export function apiError(message: string, status: number = 400) {
  return NextResponse.json(
    { success: false, data: null, error: message, meta: null },
    { status },
  );
}

export function parsePagination(searchParams: URLSearchParams) {
  const page = Math.max(1, parseInt(searchParams.get('page') ?? '1', 10));
  const limit = Math.min(100, Math.max(1, parseInt(searchParams.get('limit') ?? '20', 10)));
  return { page, limit, skip: (page - 1) * limit };
}
