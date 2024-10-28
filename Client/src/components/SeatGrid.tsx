import Seat from './Seat'

interface SeatGridProps {
    fillPercentage: number;
    zone: string;
}


export default function SeatGrid({ fillPercentage, zone }: SeatGridProps) {
    const totalSeats = 25; // 5 filas x 5 columnas
    const occupiedSeats = Math.round((fillPercentage / 100) * totalSeats);

    // Genera el estado de los asientos basado en fillPercentage
    const seats = Array.from({ length: totalSeats }, (_, index) => {
        if (index < occupiedSeats) return "occupied";
        if (index < occupiedSeats + 5) return "reserved";
        return "available";
    });

    return (
        <div className="space-y-4 p-2">
          <h2 className="text-2xl font-bold text-center mb-4">{zone}</h2>
          <div className="grid grid-cols-5 gap-1 transform scale-90">
            {seats.map((state, index) => (
              <Seat
                key={index}
                state={state as "available" | "occupied" | "reserved"}
                seatNumber={(index % 5) + 1}
                rowNumber={Math.floor(index / 5) + 1}
                showRowCol={false}
              />
            ))}
          </div>
        </div>
      );
}
