import { useLocation } from 'react-router-dom';

interface CategoryState {
  zone: string;
}

function Category() {
  const location = useLocation();
  const state = location.state as CategoryState;
  const zone = state.zone;

  return (
    <div>
      <h1>
        {zone}
      </h1>
    </div>
  );
}

export default Category;
