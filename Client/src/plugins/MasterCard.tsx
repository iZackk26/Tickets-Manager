import PaymentType from "../types/Payment";
import { LuNfc } from "react-icons/lu";

function MasterCard({ cardNumber, cardHolderName, validationDate, expirationDate, cvv }: PaymentType) {
  return (
    <div className="w-96 h-56 bg-white text-black rounded-xl overflow-hidden relative shadow-lg">
      <div className="p-6 flex flex-col justify-between h-full">
        <div className="flex justify-between items-start">
          <LuNfc className="w-6 h-6 text-gray-500" />
          <div className="text-right">
            <p className="text-xs text-gray-500">Expires</p>
            <p className="font-medium text-black">{expirationDate}</p>
          </div>
        </div>
        <div className="space-y-4">
          <p className="text-2xl tracking-wider font-semibold">{cardNumber}</p>
          <div className="flex justify-between items-center">
            <div>
              <p className="text-xs text-gray-500">Card holder</p>
              <p className="font-medium text-black">{cardHolderName}</p>
            </div>
            <div>
              <p className="text-xs text-gray-500">CVV</p>
              <p className="font-medium text-black">{cvv}</p>
            </div>
          </div>
        </div>
        <div className="flex justify-between pt-4">
          <div>
            <p className="text-xs text-gray-500">Valid From</p>
            <p className="font-medium text-black">{validationDate}</p>
          </div>
        </div>
        <div className="absolute bottom-6 right-6">
          <img src="src/assets/Mastercard-Logo.png" alt="Mastercard Logo" className="w-12 h-auto" />
        </div>
      </div>
      <div className="absolute inset-0 bg-white opacity-5 pointer-events-none">
        <svg className="w-full h-full" xmlns="http://www.w3.org/2000/svg">
          <filter id="noise">
            <feTurbulence type="fractalNoise" baseFrequency="0.80" numOctaves="4" stitchTiles="stitch" />
          </filter>
          <rect width="100%" height="100%" filter="url(#noise)" />
        </svg>
      </div>
    </div>
  );
  
}

export default MasterCard;
