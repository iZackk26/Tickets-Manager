import React from 'react';
import Seat from '../components/Seat';
import AxiosService from '../classes/AxiosService';
import ROUTES from '../constants/routes';
import BaseComponent from '../classes/BaseComponent';

interface SeatsData {
  [key: string]: string[];
}

interface Props { }

interface State {
  seatsData: SeatsData;
  zone: string;
  category: string;
}

class SeatsView extends BaseComponent<Props, State> {
  constructor(props: Props) {
    super(props);

    // Obtiene los parámetros `zone` y `category` desde la URL usando `window.location.pathname`
    const pathSegments = window.location.pathname.split('/');
    const zone = pathSegments[2] || "defaultZone";
    const category = pathSegments[3] || "defaultCategory";

    this.state = {
      seatsData: {},
      zone,
      category,
    };
  }

  componentDidMount() {
    super.componentDidMount(); // Llama al `componentDidMount` de `BaseComponent`
  }

  fetchSeatsData = async () => {
    const { zone, category } = this.state;
    try {
      const response = await AxiosService.getInstance().get(
        `${ROUTES.getSeatsByZoneAndCategory}/${zone}/${category}`
      );
      this.setState({ seatsData: response.data as SeatsData });
    } catch (error) {
      console.error('Error al cargar los datos de asientos', error);
    }
  };

  // Orden de las filas
  rowsOrder = ['w', 'x', 'y', 'z'];

  render() {
    const { seatsData } = this.state;

    return (
      <BaseComponent onMount={this.fetchSeatsData}>
        <div className="p-6">
          <h1 className="text-3xl font-bold text-center mb-10">
            Disponibilidad de Asientos
          </h1>
          <div>
            {this.rowsOrder.map((rowKey) => (
              <div key={rowKey} className="mb-4">
                <div className="flex justify-center space-x-2">
                  {seatsData[rowKey]?.map((status, seatIndex) => (
                    <Seat
                      key={seatIndex}
                      state={status.toLowerCase() as 'available' | 'reserved' | 'purchased'}
                      seatLabel={`${rowKey}:${seatIndex + 1}`} // Etiqueta personalizada
                      onClick={() => console.log(`Asiento ${rowKey}:${seatIndex + 1} clickeado`)}
                    />
                  ))}
                </div>
              </div>
            ))}
          </div>
        </div>
      </BaseComponent>
    );
  }
}

export default SeatsView;
