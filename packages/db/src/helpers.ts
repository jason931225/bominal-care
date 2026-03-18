import type { Pool, QueryResult, QueryResultRow } from 'pg';

export async function query<T extends QueryResultRow>(
  pool: Pool,
  sql: string,
  params?: unknown[],
): Promise<T[]> {
  const result: QueryResult<T> = await pool.query<T>(sql, params);
  return result.rows;
}

export async function queryOne<T extends QueryResultRow>(
  pool: Pool,
  sql: string,
  params?: unknown[],
): Promise<T | null> {
  const result: QueryResult<T> = await pool.query<T>(sql, params);
  return result.rows[0] ?? null;
}

export async function execute(
  pool: Pool,
  sql: string,
  params?: unknown[],
): Promise<number> {
  const result = await pool.query(sql, params);
  return result.rowCount ?? 0;
}

export function generateId(): string {
  return crypto.randomUUID();
}
