import React, { Component } from "react";
import Dropdown, { Option } from "react-dropdown";
import SeatType from "../types/Seat";
import AxiosService from "../classes/AxiosService";
import ROUTES from "../constants/routes";
import Seat from "../components/Seat";

interface SeatBookingState {
  option: string;
  seatsQuantity: number;
  seats: SeatType[][];
  seatPrice: number; // Añadido para el precio por asiento
}

class SeatBooking extends Component<{}, SeatBookingState> {
  options: string[] = ['A', 'B', 'C', 'D'];

  constructor(props: {}) {
    super(props);

    this.state = {
      option: this.options[0],
      seatsQuantity: 1,
      seats: [],
      seatPrice: 50, // Precio fijo por asiento
    };
  }

  handleDropdownChange = (selectedOption: Option) => {
    this.setState({ option: selectedOption.value });
  };

  handleSeatsQuantityChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    const value = parseInt(event.target.value, 10);
    this.setState({ seatsQuantity: isNaN(value) ? this.state.seatsQuantity : value });
  };

  fetchSeatsData = async () => {
    const { option, seatsQuantity } = this.state;

    try {
      const response = await AxiosService.getInstance().get(
        `${ROUTES.getSeats}/${option.toLowerCase()}/${seatsQuantity}`,
      );
      this.setState({ seats: response.data as SeatType[][] });
    } catch (error) {
      console.error("Error al cargar los datos de asientos", error);
    }
  };

  handleSeatGroupClick = (selectedSeats: SeatType[]) => {
    // Almacena los asientos seleccionados en sessionStorage
    sessionStorage.setItem('selectedSeats', JSON.stringify(selectedSeats));
  
    // Navega a la página de pago
    window.location.href = '/payment';
  };

  render() {
    const { seats, seatPrice } = this.state;

    // Cálculo del precio total basado en la cantidad de asientos
    const totalSeats = seats.flat().length;
    // total price is seat visibility * 100
    const totalPrice = totalSeats * seatPrice;

    return (
      <div className="flex flex-row items-start justify-between w-11/12">
        {/* Contenedor derecho para seleccionar las opciones de búsqueda */}
        <div className="flex flex-col items-center justify-center mr-4">
          <h1 className="font-bold text-2xl">Event Seat Finder</h1>
          <div className="flex flex-col items-start justify-center p-4 border-2 rounded-lg border-gray-300">
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
            <button
              className="bg-black text-white rounded-lg w-full p-3 mt-3"
              onClick={this.fetchSeatsData}
            >
              Search Seats
            </button>
          </div>
        </div>

        {/* Contenedor izquierdo para mostrar las respuestas de los asientos */}
        <div className="flex flex-col items-center justify-center p-4 w-full">
          <h2 className="text-xl font-bold mb-6">Top Seat Suggestions</h2>
          <div className="space-y-6">
            {seats.map((row, index) => {
              const solutionPrice = (row.reduce((acc, seat) => acc + (seat.visibility / 100 * seatPrice), 0) * 100).toFixed(2);
              return (
                <button
                  key={index}
                  className="flex flex-col items-start border rounded-lg shadow-md p-2"
                  onClick={() => this.handleSeatGroupClick(row)}
                >
                  <div className="flex flex-row items-center space-x-2">
                    {row.map((seat) => (
                      <Seat
                        key={`${seat.row}-${seat.number}`}
                        state={seat.status.toLowerCase() as "available" | "reserved" | "purchased"}
                        seatLabel={`${seat.row}-${seat.number}`}
                        visibility={seat.visibility}
                      />
                    ))}
                  </div>
                  <div className="flex justify-center w-full text-center text-sm font-semibold mt-2">
                    Solution Price: ${solutionPrice}
                  </div>
                </button>
              );
            })}
          </div>
        </div>
      </div>
    );
  }
}

export default SeatBooking;
