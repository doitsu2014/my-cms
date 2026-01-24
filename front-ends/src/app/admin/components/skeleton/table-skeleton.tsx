import React from "react";

interface TableSkeletonProps {
  rows?: number;
  columns?: number;
  showHeader?: boolean;
  className?: string;
}

const TableSkeleton: React.FC<TableSkeletonProps> = ({
  rows = 5,
  columns = 4,
  showHeader = true,
  className = "",
}) => {
  return (
    <div className={`w-full overflow-hidden rounded-lg ${className}`}>
      <div className="w-full overflow-x-auto">
        <table className="table bg-base-100">
          {showHeader && (
            <thead>
              <tr>
                {Array(columns)
                  .fill(null)
                  .map((_, idx) => (
                    <th key={idx} className="px-4 py-3">
                      <div className="skeleton h-4 w-3/4"></div>
                    </th>
                  ))}
              </tr>
            </thead>
          )}
          <tbody>
            {Array(rows)
              .fill(null)
              .map((_, rowIdx) => (
                <tr key={rowIdx}>
                  {Array(columns)
                    .fill(null)
                    .map((_, colIdx) => (
                      <td key={colIdx} className="px-4 py-3">
                        <div className="skeleton h-3 w-full"></div>
                      </td>
                    ))}
                </tr>
              ))}
          </tbody>
        </table>
      </div>
    </div>
  );
};

export default TableSkeleton;
