import Seat from './Seat';
import { useNavigate } from 'react-router-dom';

interface SeatGridProps {
  fillPercentage: number;
  zone: string;
  category: string;
}

export default function SeatGrid({ fillPercentage, zone, category }: SeatGridProps) {
  const navigate = useNavigate();
  const totalSeats = 25;
  const lowerZone = zone.toLowerCase();
  const occupiedSeats = Math.round((fillPercentage / 100) * totalSeats);

  const seats = Array.from({ length: totalSeats }, (_, index) => {
    if (index < occupiedSeats) return "occupied";
    if (index < occupiedSeats + 5) return "reserved";
    return "available";
  });

  const handleSeatClick = () => {
    navigate(`/seats/${lowerZone}/${category}`); // Navega usando la nueva ruta
  };

  return (
    <div className="space-y-4 p-2">
      <h2 className="text-2xl font-bold text-center mb-4">{category.toUpperCase()}</h2>
      <div className="grid grid-cols-5 gap-1 transform scale-90">
        {seats.map((state, index) => (
          <Seat
            key={index}
            state={state as "available" | "occupied" | "reserved"}
            seatNumber={(index % 5) + 1}
            rowNumber={Math.floor(index / 5) + 1}
            showRowCol={false}
            onClick={(state === "available" || state === "reserved") ? handleSeatClick : undefined} // Solo clickeable si es "available" o "reserved"
          />
        ))}
      </div>
    </div>
  );
}
