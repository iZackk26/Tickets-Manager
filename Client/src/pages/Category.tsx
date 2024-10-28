import React from 'react';
import ProgressBar from '../components/ProgressBar';
import SeatGrid from '../components/SeatGrid';
import AxiosService from '../classes/AxiosService';
import ROUTES from '../constants/routes';
import BaseComponent from '../classes/BaseComponent';

interface CategoryState {
  zone: string;
}

interface AvailabilityData {
  [key: string]: number;
}

interface Props { }

interface State {
  availability: AvailabilityData;
  zone: string;
}

class Category extends BaseComponent<Props, State> {
  private apiService = AxiosService.getInstance();

  constructor(props: Props) {
    super(props);
    const state = (window.history.state as { usr: CategoryState }) || { usr: { zone: "north" } };
    this.state = {
      availability: {},
      zone: state.usr.zone,
    };
  }

  componentDidMount() {
    super.componentDidMount(); // Llama a componentDidMount de BaseComponent
  }

  fetchAvailabilityData = async () => {
    try {
      const url = `${ROUTES.availableSeatsByCategory}/${this.state.zone.toLowerCase()}`;
      const response = await this.apiService.get(url);
      console.log(response.data);
      this.setState({ availability: response.data as AvailabilityData });
    } catch (error) {
      console.error("Error al obtener los datos de disponibilidad", error);
    }
  };

  calculatePercentage = (availableSeats: number) =>
    Math.round(((40 - availableSeats) / 40) * 100);

  render() {
    const { availability, zone } = this.state;

    return (
      <BaseComponent onMount={this.fetchAvailabilityData}>
        <div className="h-screen flex flex-col items-center p-4">
          <div className="text-3xl font-bold mb-4">Disponibilidad de Asientos - Zona {zone}</div>

          <div className="flex flex-1 w-full items-center justify-center">
            <div className="flex-1 p-2">
              <SeatGrid fillPercentage={this.calculatePercentage(availability['a'] || 0)} zone={zone} category="a" />
              <ProgressBar percentage={this.calculatePercentage(availability['a'] || 0)} />
            </div>
            <div className="flex-1 p-2">
              <SeatGrid fillPercentage={this.calculatePercentage(availability['b'] || 0)} zone={zone} category="b" />
              <ProgressBar percentage={this.calculatePercentage(availability['b'] || 0)} />
            </div>
            <div className="flex-1 p-2">
              <SeatGrid fillPercentage={this.calculatePercentage(availability['c'] || 0)} zone={zone} category="c" />
              <ProgressBar percentage={this.calculatePercentage(availability['c'] || 0)} />
            </div>
            <div className="flex-1 p-2">
              <SeatGrid fillPercentage={this.calculatePercentage(availability['d'] || 0)} zone={zone} category="d" />
              <ProgressBar percentage={this.calculatePercentage(availability['d'] || 0)} />
            </div>
          </div>
        </div>
      </BaseComponent>
    );
  }
}

export default Category;
