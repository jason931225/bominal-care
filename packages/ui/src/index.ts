// Utilities
export { cn, formatDate, formatCurrency, formatTime, formatPhoneNumber } from './lib/utils';

// Tailwind Preset
export { default as tailwindPreset } from './tailwind-preset';

// Providers
export { ThemeProvider, useTheme } from './providers/ThemeProvider';

// Primitives
export { Button, ButtonLink, buttonVariants } from './components/primitives/Button';
export type { ButtonVariants } from './components/primitives/Button';
export { Input } from './components/primitives/Input';
export { Textarea } from './components/primitives/Textarea';
export { Select } from './components/primitives/Select';
export { Checkbox } from './components/primitives/Checkbox';
export { RadioGroup } from './components/primitives/RadioGroup';
export { Switch } from './components/primitives/Switch';
export { Label } from './components/primitives/Label';
export { Badge } from './components/primitives/Badge';
export { Avatar } from './components/primitives/Avatar';

// Layout
export { AppShell } from './components/layout/AppShell';
export { TopBar } from './components/layout/TopBar';
export { SideNav } from './components/layout/SideNav';
export { BottomNavBar } from './components/layout/BottomNavBar';
export { PageHeader } from './components/layout/PageHeader';
export { Section } from './components/layout/Section';
export { CardGrid } from './components/layout/CardGrid';

// Data Display
export { DataTable } from './components/data-display/DataTable';
export type { ColumnDef } from './components/data-display/DataTable';
export { Timeline } from './components/data-display/Timeline';
export { StatWidget } from './components/data-display/StatWidget';
export { StatusBadge } from './components/data-display/StatusBadge';
export type { StatusMap } from './components/data-display/StatusBadge';
export { DataCard } from './components/data-display/DataCard';
export { EmptyState } from './components/data-display/EmptyState';

// Forms
export { FormField } from './components/forms/FormField';
export { SearchInput } from './components/forms/SearchInput';
export { DatePicker } from './components/forms/DatePicker';
export { FileUpload } from './components/forms/FileUpload';
export { StepWizard } from './components/forms/StepWizard';
export type { WizardStep } from './components/forms/StepWizard';
export { PhoneInput } from './components/forms/PhoneInput';

// Feedback
export { ToastProvider, useToast } from './components/feedback/Toast';
export { AlertBanner } from './components/feedback/AlertBanner';
export { ConfirmDialog } from './components/feedback/ConfirmDialog';
export { LoadingSpinner } from './components/feedback/LoadingSpinner';
export { Skeleton } from './components/feedback/Skeleton';
export { ErrorPage } from './components/feedback/ErrorPage';
export { LoadingPage } from './components/feedback/LoadingPage';
export { NotFoundPage } from './components/feedback/NotFoundPage';

// Senior
export { LargeButton } from './components/senior/LargeButton';
export { MedicationCard } from './components/senior/MedicationCard';
export { ScheduleBlock } from './components/senior/ScheduleBlock';
export { ConsentToggle } from './components/senior/ConsentToggle';
export { EmergencyButton } from './components/senior/EmergencyButton';
export { QuickAction } from './components/senior/QuickAction';

