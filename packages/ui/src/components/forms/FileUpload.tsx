'use client';
import { useState, useCallback, type ChangeEvent, type DragEvent } from 'react';
import { cn } from '../../lib/utils';

interface FileUploadProps {
  accept?: string;
  multiple?: boolean;
  onFilesChange: (files: File[]) => void;
  className?: string;
  label?: string;
  helperText?: string;
  error?: string;
}

export function FileUpload({ accept, multiple, onFilesChange, className, label, helperText, error }: FileUploadProps) {
  const [dragging, setDragging] = useState(false);
  const [files, setFiles] = useState<File[]>([]);

  const addFiles = useCallback(
    (newFiles: FileList | null) => {
      if (!newFiles) return;
      const list = Array.from(newFiles);
      const updated = multiple ? [...files, ...list] : list;
      setFiles(updated);
      onFilesChange(updated);
    },
    [files, multiple, onFilesChange],
  );

  function removeFile(index: number) {
    const updated = files.filter((_, i) => i !== index);
    setFiles(updated);
    onFilesChange(updated);
  }

  function handleDrop(e: DragEvent<HTMLDivElement>) {
    e.preventDefault();
    setDragging(false);
    addFiles(e.dataTransfer.files);
  }

  function handleChange(e: ChangeEvent<HTMLInputElement>) {
    addFiles(e.target.files);
    e.target.value = '';
  }

  return (
    <div className={cn('flex flex-col gap-2', className)}>
      {label && <span className="text-sm font-medium text-gray-700">{label}</span>}
      <div
        onDragOver={(e) => { e.preventDefault(); setDragging(true); }}
        onDragLeave={() => setDragging(false)}
        onDrop={handleDrop}
        className={cn(
          'relative flex flex-col items-center justify-center rounded-lg border-2 border-dashed p-8 text-center transition-colors',
          dragging ? 'border-primary-500 bg-primary-50' : 'border-gray-300 hover:border-gray-400',
          error && 'border-danger-500',
        )}
      >
        <svg className="mb-2 h-8 w-8 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24" aria-hidden="true">
          <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={1.5} d="M4 16v2a2 2 0 002 2h12a2 2 0 002-2v-2M12 12V4m0 0L8 8m4-4 4 4" />
        </svg>
        <p className="text-sm text-gray-600">
          파일을 드래그하거나{' '}
          <label className="cursor-pointer text-primary-600 hover:underline">
            여기를 클릭하세요
            <input type="file" accept={accept} multiple={multiple} className="sr-only" onChange={handleChange} />
          </label>
        </p>
        {helperText && <p className="mt-1 text-xs text-gray-400">{helperText}</p>}
      </div>
      {error && <p className="text-sm text-danger-500">{error}</p>}
      {files.length > 0 && (
        <ul className="flex flex-col gap-1">
          {files.map((file, idx) => (
            <li key={idx} className="flex items-center justify-between rounded-md border border-gray-200 px-3 py-2 text-sm">
              <span className="truncate text-gray-700">{file.name}</span>
              <button
                type="button"
                onClick={() => removeFile(idx)}
                className="ml-2 flex-shrink-0 text-gray-400 hover:text-danger-500 transition-colors"
                aria-label={`${file.name} 삭제`}
              >
                ✕
              </button>
            </li>
          ))}
        </ul>
      )}
    </div>
  );
}
