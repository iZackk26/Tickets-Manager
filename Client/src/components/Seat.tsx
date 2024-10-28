import { PiSeatLight } from 'react-icons/pi';

interface SeatProps {
  state: 'available' | 'reserved' | 'purchased';
  seatLabel: string; // Etiqueta personalizada, por ejemplo: w:1, x:2
  onClick?: () => void;
}

export default function Seat({ state, seatLabel, onClick }: SeatProps) {
  const isAvailable = state === 'available';
  const isReserved = state === 'reserved';
  const isPurchased = state === 'purchased';

  return (
    <div className="flex flex-col items-center">
      <button
        onClick={onClick}
        className={`relative h-8 w-8 overflow-hidden rounded-lg transition-all duration-200 ${
          isAvailable
            ? 'border-indigo-600 text-indigo-600 hover:bg-indigo-600 hover:text-white'
            : isReserved
            ? 'border-blue-400 text-blue-400 cursor-not-allowed opacity-75'
            : 'border-gray-400 text-gray-400 cursor-not-allowed opacity-50'
        }`}
        disabled={isPurchased} // Deshabilitar si el asiento está comprado
      >
        <PiSeatLight className="absolute top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2 text-lg" />
      </button>
      <span className="text-xs mt-1 text-gray-500">{seatLabel}</span> {/* Muestra la etiqueta personalizada */}
    </div>
  );
}
