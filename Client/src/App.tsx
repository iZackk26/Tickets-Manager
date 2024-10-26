import { useState, useEffect } from 'react';
import DynamicPluginLoader from './DinamicPluginLoader';
import AxiosService from './classes/AxiosService';
import ROUTES from './constants/routes';

function App() {
  const apiService = AxiosService.getInstance();
  const [loadPlugin, setLoadPlugin] = useState(false);

  const handleButtonClick = () => {
    setLoadPlugin(!loadPlugin);
  };

  const fetchData = async () => {
    try {
      const response = await apiService.get(ROUTES.availableSeats);
      console.log('Datos recibidos:', response.data);
    } catch (error) {
      console.error('Error al obtener datos:', error);
    }
  };

  useEffect(() => {
    fetchData();
  }, [])

  return (
    <div className="flex items-center justify-center w-screen h-screen">
      <div>
        <button
          className="px-2 py-1 bg-red-500 rounded-xl overflow-hidden"
          onClick={handleButtonClick}
        >
          Press me to load a plug-in
        </button>

        {loadPlugin && <DynamicPluginLoader />}
      </div>
    </div>
  );
}

export default App;
