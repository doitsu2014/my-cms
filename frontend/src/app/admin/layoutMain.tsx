import { useLayout } from './layoutContext';
import TopBar from './components/top-bar';

export function MainLayout({ children }: { children: React.ReactNode }) {
  const { isLoading } = useLayout();
  return (
    <main className="flex-1 px-6 bg-base-100 min-h-screen">
      <TopBar />
      {isLoading && (
        <div className="flex justify-center items-center h-96">
          <span className="loading loading-spinner loading-lg text-primary"></span>
        </div>
      )}
      {children}
    </main>
  );
}
