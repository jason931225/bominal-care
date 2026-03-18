import { z } from "zod";

// -----------------------------------------------------------------------------
// PaginationMeta
// -----------------------------------------------------------------------------

export const PaginationMetaSchema = z.object({
  total: z.number().int().nonnegative(),
  page: z.number().int().positive(),
  limit: z.number().int().positive(),
  totalPages: z.number().int().nonnegative(),
});

export type PaginationMeta = z.infer<typeof PaginationMetaSchema>;

// -----------------------------------------------------------------------------
// PaginationParams
// -----------------------------------------------------------------------------

export const PaginationParamsSchema = z.object({
  page: z.number().int().positive().default(1),
  limit: z.number().int().positive().max(100).default(20),
});

export type PaginationParams = z.infer<typeof PaginationParamsSchema>;

// -----------------------------------------------------------------------------
// ApiResponse<T>
// -----------------------------------------------------------------------------

export const ApiResponseSchema = <T extends z.ZodTypeAny>(dataSchema: T) =>
  z.object({
    success: z.boolean(),
    data: dataSchema.nullable(),
    error: z.string().nullable(),
    meta: PaginationMetaSchema.optional(),
  });

export type ApiResponse<T> = {
  readonly success: boolean;
  readonly data: T | null;
  readonly error: string | null;
  readonly meta?: PaginationMeta;
};

// -----------------------------------------------------------------------------
// IdParam
// -----------------------------------------------------------------------------

export const IdParamSchema = z.object({
  id: z.string().min(1),
});

export type IdParam = z.infer<typeof IdParamSchema>;

// -----------------------------------------------------------------------------
// DateRange
// -----------------------------------------------------------------------------

export const DateRangeSchema = z.object({
  from: z.coerce.date(),
  to: z.coerce.date(),
});

export type DateRange = z.infer<typeof DateRangeSchema>;
