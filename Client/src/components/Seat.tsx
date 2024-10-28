import { PiSeatLight } from "react-icons/pi";

interface SeatProps {
  state: "available" | "occupied" | "reserved";
  seatNumber: number;
  rowNumber: number;
  showRowCol?: boolean; // Nueva prop opcional
}

export default function Seat({ state, seatNumber, rowNumber, showRowCol = true }: SeatProps) {
  const isAvailable = state === "available";
  const isOccupied = state === "occupied";
  const isReserved = state === "reserved";

  return (
    <div className="flex flex-col items-center">
      <button
        className={`relative h-8 w-8 overflow-hidden rounded-lg transition-all duration-200 ${
          isAvailable
            ? "border-indigo-600 text-indigo-600 hover:bg-indigo-600 hover:text-white hover:shadow-indigo-600"
            : isReserved
            ? "border-blue-400 text-blue-400 cursor-not-allowed opacity-75 hover:bg-blue-200"
            : "border-gray-400 text-gray-400 cursor-not-allowed opacity-50"
        }`}
        disabled={!isAvailable}
      >
        <PiSeatLight className="absolute top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2 text-lg" />
      </button>
      {/* Mostrar fila y asiento solo si showRowCol es true */}
      {showRowCol && (
        <span className="text-xs mt-1 text-gray-500">{`${rowNumber}:${seatNumber}`}</span>
      )}
    </div>
  );
}
