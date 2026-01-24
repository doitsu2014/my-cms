import { createContext, useContext, useState } from "react";

interface LayoutContextProps {
  isLoading: boolean;
  setLayoutLoading: (loading: boolean) => void;
}

const LayoutContext = createContext<LayoutContextProps | undefined>(undefined);

export const LayoutProvider = ({ children }: { children: React.ReactNode }) => {
  const [isLoading, setLayoutLoading] = useState(false);

  return (
    <LayoutContext.Provider value={{ isLoading, setLayoutLoading }}>
      {children}
    </LayoutContext.Provider>
  );
};

export const useLayout = () => {
  const context = useContext(LayoutContext);
  if (!context) {
    throw new Error("useLayout must be used within a LayoutProvider");
  }
  return context;
};
