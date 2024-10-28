import React, { Component } from "react";
import Dropdown, { Option } from "react-dropdown";
import 'react-dropdown/style.css';
import SeatType from "../types/Seat";
import AxiosService from "../classes/AxiosService";
import ROUTES from "../constants/routes";

interface SeatBookingState {
  option: string;
  seatsQuantity: number;
  seats: SeatType[][];
}

class SeatBooking extends Component<{}, SeatBookingState> {
  // Define las opciones como propiedad de clase
  options: string[] = ['A', 'B', 'C', 'D'];

  constructor(props: {}) {
    super(props);

    // Define el estado inicial en el constructor
    this.state = {
      option: this.options[0],
      seatsQuantity: 1, // Cantidad de asientos inicial
      seats: [], // Inicialmente vacío
    };
  }

  // Método para manejar el cambio del dropdown
  handleDropdownChange = (selectedOption: Option) => {
    this.setState({ option: selectedOption.value });
  };

  // Método para manejar el cambio en el input de cantidad de asientos
  handleSeatsQuantityChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    const value = parseInt(event.target.value, 10);
    this.setState({ seatsQuantity: isNaN(value) ? this.state.seatsQuantity : value });
  };

  // Método para realizar la llamada a la API con el cuerpo de la solicitud
  fetchSeatsData = async () => {
    const { option, seatsQuantity } = this.state;

    try {
      const response = await AxiosService.getInstance().get(
        `${ROUTES.getSeats}/${option.toLowerCase()}/${seatsQuantity}`,
      );
      this.setState({ seats: response.data as SeatType[][] });
      console.log(response.data)
    } catch (error) {
      console.error('Error al cargar los datos de asientos', error);
    }
  };

  render() {
    return (
      <div className="flex flex-row items-center justify-between w-11/12">
        <div className="flex flex-col items-center justify-center">
          <h1 className="font-bold text-2xl">Event Seat Finder</h1>
          <div className="flex flex-col items-start justify-center p-3 border-2 border-gray-300">
            <p>Search for Seats</p>
            <p>Select your preferences to find available seats</p>
            <p>Stadium Category</p>
            <Dropdown
              options={this.options}
              value={this.state.option}
              onChange={this.handleDropdownChange}
              placeholder="Select an option"
            />
            <input
              type="number"
              className="border-0 border-b-2 w-full border-gray-500 p-1 focus:border-b-2 focus:border-black focus:ring-0 focus:outline-0 lg:w-full"
              placeholder="Number of seats"
              value={this.state.seatsQuantity}
              onChange={this.handleSeatsQuantityChange}
              min="1"
            />
            <button className="bg-black text-white rounded-lg w-full p-3 mt-3" onClick={this.fetchSeatsData}>
              Search Seats
            </button>
          </div>
        </div>
      </div>
    );
  }
}

export default SeatBooking;
