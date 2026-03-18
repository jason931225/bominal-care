export function LoadingPage() {
  return (
    <div
      className="flex min-h-[60vh] flex-col items-center justify-center gap-4"
      role="status"
      aria-label="로딩 중..."
    >
      <div
        className="h-10 w-10 animate-spin rounded-full border-4 border-gray-200 border-t-primary-600"
        aria-hidden="true"
      />
      <p className="text-sm font-medium text-gray-500">로딩 중...</p>
    </div>
  );
}
