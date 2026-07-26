interface CategoryLabelProps {
  children: React.ReactNode;
}

const CategoryLabel = ({ children }: CategoryLabelProps) => (
  <span className="category-label">{children}</span>
);

export default CategoryLabel;
