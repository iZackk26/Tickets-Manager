import { useEffect, useState } from 'react';
import { useLocation } from 'react-router-dom';
import ProgressBar from '../components/ProgressBar';
import SeatGrid from '../components/SeatGrid';
import AxiosService from '../classes/AxiosService';
import ROUTES from '../constants/routes';

interface CategoryState {
  zone: string;
}

// Define el tipo de los datos de disponibilidad
interface AvailabilityData {
  [key: string]: number;
}

export default function Category() {
  const apiService = AxiosService.getInstance();
  const location = useLocation();
  const state = location.state as CategoryState;
  const zone = state.zone;

  const [availability, setAvailability] = useState<AvailabilityData>({});

  useEffect(() => {
    const fetchAvailabilityData = async () => {
      try {
        // Construir la URL de la solicitud usando el endpoint y la zona
        const url = `${ROUTES.availableSeatsByCategory}/${zone.toLowerCase()}`;
        const response = await apiService.get(url);
        console.log(response.data);

        setAvailability(response.data as AvailabilityData); // Actualizar el estado con los datos obtenidos
      } catch (error) {
        console.error("Error al obtener los datos de disponibilidad", error);
      }
    };

    fetchAvailabilityData();
  }, [apiService, zone]);

  // Calcular el porcentaje de asientos disponibles para cada categoría
  const calculatePercentage = (availableSeats: number) => Math.round(((40 - availableSeats) / 40) * 100);



  return (
    <div className="h-screen flex flex-col items-center p-4">
      {/* Contenedor de zone en la parte superior */}
      <div className="text-3xl font-bold mb-4">Disponibilidad de Asientos - Zona {zone}</div>

      {/* Contenido principal centrado */}
      <div className="flex flex-1 w-full items-center justify-center">
        {/* Renderizar cada categoría con porcentaje calculado */}
        <div className="flex-1 p-2">
          <SeatGrid fillPercentage={calculatePercentage(availability['a'] || 0)} zone={zone} category='a' />
          <ProgressBar percentage={calculatePercentage(availability['a'] || 0)} />
        </div>
        <div className="flex-1 p-2">
          <SeatGrid fillPercentage={calculatePercentage(availability['b'] || 0)} zone={zone} category='b' />
          <ProgressBar percentage={calculatePercentage(availability['b'] || 0)} />
        </div>
        <div className="flex-1 p-2">
          <SeatGrid fillPercentage={calculatePercentage(availability['c'] || 0)} zone={zone} category='c' />
          <ProgressBar percentage={calculatePercentage(availability['c'] || 0)} />
        </div>
        <div className="flex-1 p-2">
          <SeatGrid fillPercentage={calculatePercentage(availability['d'] || 0)} zone={zone} category='d' />
          <ProgressBar percentage={calculatePercentage(availability['d'] || 0)} />
        </div>
      </div>
    </div>
  );
}
