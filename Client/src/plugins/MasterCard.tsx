import { useState } from "react";
import { LuNfc } from "react-icons/lu";
import PaymentType from "../types/Payment";

function MasterCard() {
  // Estado para almacenar los valores de los campos
  const [cardNumber, setCardNumber] = useState("");
  const [cardHolderName, setCardHolderName] = useState("");
  const [validationDate, setValidationDate] = useState("");
  const [expirationDate, setExpirationDate] = useState("");
  const [cvv, setCvv] = useState("");

  return (
    <div className="w-96 h-56 bg-white text-black rounded-xl overflow-hidden relative shadow-lg">
      <div className="p-6 flex flex-col justify-between h-full">
        <div className="flex justify-between items-start">
          <LuNfc className="w-6 h-6 text-gray-500" />
          <div className="text-right">
            <p className="text-xs text-gray-500">Expires</p>
            <input
              type="text"
              value={expirationDate}
              onChange={(e) => setExpirationDate(e.target.value)}
              placeholder="MM/YY"
              className="bg-transparent border-none text-right text-black font-medium w-16 text-xs focus:outline-none"
            />
          </div>
        </div>
        <div className="space-y-4">
          <input
            type="text"
            value={cardNumber}
            onChange={(e) => setCardNumber(e.target.value)}
            placeholder="1234 5678 9012 3456"
            maxLength={19}
            className="bg-transparent border-none text-2xl tracking-wider font-semibold focus:outline-none w-full"
          />
          <div className="flex justify-between items-center">
            <div>
              <p className="text-xs text-gray-500">Card holder</p>
              <input
                type="text"
                value={cardHolderName}
                onChange={(e) => setCardHolderName(e.target.value)}
                placeholder="John Doe"
                className="bg-transparent border-none font-medium text-black focus:outline-none w-full text-sm"
              />
            </div>
            <div>
              <p className="text-xs text-gray-500">CVV</p>
              <input
                type="text"
                value={cvv}
                onChange={(e) => setCvv(e.target.value)}
                placeholder="123"
                maxLength={3}
                className="bg-transparent border-none font-medium text-black focus:outline-none w-12 text-sm"
              />
            </div>
          </div>
        </div>
        <div className="flex justify-between pt-4">
          <div>
            <p className="text-xs text-gray-500">Valid From</p>
            <input
              type="text"
              value={validationDate}
              onChange={(e) => setValidationDate(e.target.value)}
              placeholder="MM/YY"
              className="bg-transparent border-none font-medium text-black focus:outline-none w-16 text-xs"
            />
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
