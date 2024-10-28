import { useParams } from 'react-router-dom';
import { useState, useEffect } from 'react';
import Seat from '../components/Seat';
import AxiosService from '../classes/AxiosService';
import ROUTES from '../constants/routes';

interface SeatsData {
  [key: string]: string[];
}

export default function SeatsView() {
  const { zone, category } = useParams<{ zone: string; category: string }>();
  const [seatsData, setSeatsData] = useState<SeatsData>({});

  useEffect(() => {
    const fetchSeatsData = async () => {
      try {
        const response = await AxiosService.getInstance().get(
          `${ROUTES.getSeatsByZoneAndCategory}/${zone}/${category}`
        );
        setSeatsData(response.data as SeatsData);
      } catch (error) {
        console.error('Error al cargar los datos de asientos', error);
      }
    };

    fetchSeatsData();
  }, [zone, category]);

  // Filas en el orden especificado (w, x, y, z)
  const rowsOrder = ['w', 'x', 'y', 'z'];

  return (
    <div className="p-6">
      <h1 className="text-3xl font-bold text-center mb-10">
        Disponibilidad de Asientos
      </h1>
      <div>
        {rowsOrder.map((rowKey) => (
          <div key={rowKey} className="mb-4">
            <div className="flex justify-center space-x-2">
              {seatsData[rowKey]?.map((status, seatIndex) => (
                <Seat
                  key={seatIndex}
                  state={status.toLowerCase() as 'available' | 'reserved' | 'purchased'}
                  seatLabel={`${rowKey}:${seatIndex + 1}`} // Pasamos w:1, x:1, etc. como etiqueta personalizada
                  onClick={() => console.log(`Asiento ${rowKey}:${seatIndex + 1} clickeado`)}
                />
              ))}
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}
