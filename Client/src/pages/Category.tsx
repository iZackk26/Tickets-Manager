import { useLocation } from 'react-router-dom';
import ProgressBar from '../components/ProgressBar';
import IconButton from '@mui/material/IconButton';
import AddShoppingCartIcon from '@mui/icons-material/AddShoppingCart';
import SeatGrid from '../components/SeatGrid';

interface CategoryState {
  zone: string;
}

function Category() {
  const location = useLocation();
  const state = location.state as CategoryState;
  const zone = state.zone;

  return (
    <div className="h-screen flex flex-col items-center p-4">
      {/* Contenedor de zone en la parte superior */}
      <div className="text-3xl font-bold mb-4">{zone}</div>
  
      {/* Contenido principal centrado */}
      <div className="flex flex-1 w-full items-center justify-center">
        <div className="flex-1 p-2">
          <SeatGrid fillPercentage={60} zone="Category A" />
          <ProgressBar percentage={75} />
          <div className="flex items-center justify-between">
            <span className="ml-2 text-lg font-semibold">Buy Seats</span>
            <IconButton color="primary" aria-label="add to shopping cart">
              <AddShoppingCartIcon />
            </IconButton>
          </div>
        </div>
        <div className="flex-1 p-2">
          <SeatGrid fillPercentage={45} zone="Category B" />
          <ProgressBar percentage={50} />
          <div className="flex items-center justify-between">
            <span className="ml-2 text-lg font-semibold">Buy Seats</span>
            <IconButton color="primary" aria-label="add to shopping cart">
              <AddShoppingCartIcon />
            </IconButton>
          </div>
        </div>
        <div className="flex-1 p-2">
          <SeatGrid fillPercentage={30} zone="Category C" />
          <ProgressBar percentage={25} />
          <div className="flex items-center justify-between">
            <span className="ml-2 text-lg font-semibold">Buy Seats</span>
            <IconButton color="primary" aria-label="add to shopping cart">
              <AddShoppingCartIcon />
            </IconButton>
          </div>
        </div>
        <div className="flex-1 p-2">
          <SeatGrid fillPercentage={90} zone="Category D" />
          <ProgressBar percentage={85} />
          <div className="flex items-center justify-between">
            <span className="ml-2 text-lg font-semibold">Buy Seats</span>
            <IconButton color="primary" aria-label="add to shopping cart">
              <AddShoppingCartIcon />
            </IconButton>
          </div>
        </div>
      </div>
    </div>
  );
  
}

export default Category;
